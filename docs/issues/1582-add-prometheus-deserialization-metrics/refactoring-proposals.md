# Refactoring Proposals: `metric_collection/prometheus.rs`

Ordered from **least effort / biggest impact** to **most effort / lower impact**.

---

## 1. Extract the duplicated family-parsing loop using a trait

**Effort**: low | **Impact**: high

The `Counter` and `Gauge` arms inside `from_prometheus` are structurally identical
(~20 lines each). The only difference is which domain type is extracted from the
parser's `PrometheusValue`. We can express that difference as a small trait — one
implementation per domain type — and dispatch by type rather than by passing a
function or closure as an argument.

### Step 1 — Define the conversion trait

Each domain type that can be deserialized from a Prometheus sample value implements
this trait:

```rust
trait FromPrometheusValue: Sized {
    fn from_prometheus_value(
        family_name: &str,
        value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError>;
}

impl FromPrometheusValue for Counter {
    fn from_prometheus_value(
        family_name: &str,
        value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError> {
        // body of the existing `counter_value_from_prom`
    }
}

impl FromPrometheusValue for Gauge {
    fn from_prometheus_value(
        family_name: &str,
        value: &openmetrics_parser::PrometheusValue,
    ) -> Result<Self, PrometheusDeserializationError> {
        // body of the existing `gauge_value_from_prom`
    }
}
```

The two free functions `counter_value_from_prom` and `gauge_value_from_prom` are
removed — their bodies move into the trait `impl` blocks.

### Step 2 — Generic helper with no closure

```rust
fn parse_family_samples<T: FromPrometheusValue>(
    family_name: &str,
    family: &openmetrics_parser::PrometheusFamily<'_>,
    now: DurationSinceUnixEpoch,
) -> Result<Metric<T>, PrometheusDeserializationError> {
    let label_names = Arc::new(family.get_label_names().to_vec());
    let mut samples = Vec::new();

    for parser_sample in family.iter_samples() {
        let parser_label_set =
            openmetrics_parser::LabelSet::new(Arc::clone(&label_names), parser_sample)
                .map_err(|e| PrometheusDeserializationError::LabelConversion {
                    metric_name: family_name.to_owned(),
                    message: e.to_string(),
                })?;
        let label_set = convert_openmetrics_label_set(family_name, parser_label_set)?;
        let value = T::from_prometheus_value(family_name, &parser_sample.value)?;
        let time = parser_sample
            .timestamp
            .map_or(now, |t| parse_prometheus_timestamp(t, now));
        samples.push(Sample::new(value, time, label_set));
    }

    let metric_name = MetricName::new(family_name);
    let description = description_from_help(&family.help);
    Ok(Metric::new(
        metric_name,
        None,
        description,
        build_sample_collection(samples)?,
    ))
}
```

### Step 3 — Type-driven dispatch at the call site

```rust
openmetrics_parser::PrometheusType::Counter => {
    counter_metrics.push(parse_family_samples::<Counter>(family_name, family, now)?);
}
openmetrics_parser::PrometheusType::Gauge => {
    gauge_metrics.push(parse_family_samples::<Gauge>(family_name, family, now)?);
}
```

### Why this approach (vs. a closure parameter)

- The call site has **no closure** to read; the variant is selected by the type
  parameter, which reads naturally as `parse_family_samples::<Counter>(...)`.
- The conversion logic stays **co-located with the domain type** that owns it
  (via the `impl` block), instead of living in a free helper passed by name.
- Each `FromPrometheusValue` implementation is **independently testable**
  without going through `from_prometheus`.
- The trait is the natural foundation for Proposal 6: it can be replaced by — or
  named as — `TryFrom<(&str, &openmetrics_parser::PrometheusValue)>` if we prefer
  a fully standard-library trait. If you adopt this proposal, Proposal 6 may
  collapse into it (or be skipped entirely).

### Alternatives considered

- **Closure / `Fn` parameter** — works, but `parse_family_samples(family_name, family, now, counter_value_from_prom)?`
  is harder to read and IDE jump-to-definition lands on the helper rather than on
  the conversion logic. Rejected.
- **`fn` pointer parameter** — same readability problem as a closure; just spells
  out the type explicitly. Rejected.
- **Macro** — avoids generics but is harder to read and tool-friendly than a
  trait. Rejected unless we want to escape generics for unrelated reasons.
- **Do nothing / accept duplication** — legitimate if we are confident no further
  metric kinds will be added and the two arms will not diverge. Acceptable
  fallback, but the trait costs little and removes the duplication cleanly.

---

## 2. Name the float-guard condition

**Effort**: low | **Impact**: medium

The match guard in `counter_value_from_prom` is a four-clause boolean expression that
is hard to read at a glance:

```rust
// Before
if value.is_finite() && value >= 0.0 && value.fract() == 0.0 && value < 18_446_744_073_709_551_616.0
```

Extract it into a named predicate that documents the intent:

```rust
/// Returns `true` if `v` is a non-negative, whole number that fits in a `u64`.
fn is_whole_u64_representable(v: f64) -> bool {
    const FIRST_UNREPRESENTABLE: f64 = 18_446_744_073_709_551_616.0; // 2^64
    v.is_finite() && v >= 0.0 && v.fract() == 0.0 && v < FIRST_UNREPRESENTABLE
}
```

The guard becomes `if is_whole_u64_representable(value)`, and the predicate can be
tested directly and reused across counter-parsing logic.

---

## 3. Extract `description_from_help` helper

**Effort**: low | **Impact**: low–medium

The same `if help.is_empty() { None } else { Some(...) }` pattern would appear in
every family arm if the loop were generalized (see proposal 1). Extract it once:

```rust
fn description_from_help(help: &str) -> Option<MetricDescription> {
    if help.is_empty() {
        None
    } else {
        Some(MetricDescription::new(help))
    }
}
```

Alternatively, add `Option::filter` + `map`:

```rust
Some(help).filter(|h| !h.is_empty()).map(MetricDescription::new)
```

---

## 4. Use `Cow<str>` for input normalization

**Effort**: low | **Impact**: readability

The current pattern requires declaring `normalized` before the `if` to satisfy the
borrow checker:

```rust
let normalized;
let input = if input.ends_with('\n') {
    input
} else {
    normalized = format!("{input}\n");
    normalized.as_str()
};
```

Using `std::borrow::Cow` removes the two-statement idiom and names the intent:

```rust
fn ensure_trailing_newline(s: &str) -> Cow<'_, str> {
    if s.ends_with('\n') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("{s}\n"))
    }
}
```

`from_prometheus` starts with `let input = ensure_trailing_newline(input);` which
reads naturally and is independently testable.

---

## 5. Return `Option` from `parse_prometheus_timestamp` instead of a fallback

**Effort**: low | **Impact**: readability + testability

The current signature bakes the fallback strategy into the function:

```rust
pub(super) fn parse_prometheus_timestamp(
    t: f64,
    fallback: DurationSinceUnixEpoch,
) -> DurationSinceUnixEpoch
```

This makes tests that want to verify "invalid timestamp → None" awkward because they
must supply a sentinel fallback and then check equality. A cleaner API is:

```rust
/// Returns `None` if `t` is non-finite, negative, or would overflow `u64` seconds.
pub(super) fn parse_prometheus_timestamp(t: f64) -> Option<DurationSinceUnixEpoch>
```

The caller uses `.unwrap_or(now)`, which makes the fallback behavior explicit at the
call site:

```rust
let time = parser_sample
    .timestamp
    .and_then(parse_prometheus_timestamp)  // None if invalid
    .unwrap_or(now);
```

Tests become cleaner (`assert_eq!(parse_prometheus_timestamp(-1.0), None)`) and the
function has a single responsibility.

---

## 6. Use `TryFrom` / `TryInto` for `Counter` and `Gauge` extraction

**Effort**: medium | **Impact**: idiomatic Rust + testability

> **Note**: if Proposal 1 is adopted, this proposal can either be skipped or used
> to _replace_ the custom `FromPrometheusValue` trait with the standard `TryFrom`.

`counter_value_from_prom` and `gauge_value_from_prom` are conversion functions from
a parser value type to a domain type. Standard Rust idiom for fallible conversions is
`TryFrom`. The barrier is that the error variants need `metric_name` context.

One approach: a local wrapper type that carries the context:

```rust
struct NamedValue<'a> {
    family_name: &'a str,
    value: &'a openmetrics_parser::PrometheusValue,
}

impl TryFrom<NamedValue<'_>> for Counter {
    type Error = PrometheusDeserializationError;

    fn try_from(nv: NamedValue<'_>) -> Result<Self, Self::Error> {
        // existing counter_value_from_prom logic
    }
}
```

Call site: `Counter::try_from(NamedValue { family_name, value: &parser_sample.value })?`

This removes the `_from_prom` naming suffix, unifies extraction under one trait, and
makes dispatch type-driven rather than name-driven.

---

## 7. Centralize error mapping in the error type

**Effort**: low | **Impact**: small but consistent

`collection_error` is a free function that constructs a specific error variant. The
standard Rust approach is to implement `From<CollectionError> for PrometheusDeserializationError`
(or a specific inner error type) so `.map_err(Into::into)` / `?` does the conversion
automatically and there is no helper to name and remember.

Concretely:

```rust
impl From<MetricKindCollectionError> for PrometheusDeserializationError {
    fn from(e: MetricKindCollectionError) -> Self {
        Self::CollectionError { message: e.to_string() }
    }
}
```

`build_metric_collection` then becomes:

```rust
fn build_metric_collection(
    counter_metrics: Vec<Metric<Counter>>,
    gauge_metrics: Vec<Metric<Gauge>>,
) -> Result<MetricCollection, PrometheusDeserializationError> {
    let counters = MetricKindCollection::new(counter_metrics)?;
    let gauges  = MetricKindCollection::new(gauge_metrics)?;
    Ok(MetricCollection::new(counters, gauges)?)
}
```

Whether this is worthwhile depends on how widely `PrometheusDeserializationError` is
used outside the Prometheus layer.

---

## 8. Decompose `from_prometheus` into a two-stage pipeline

**Effort**: high | **Impact**: highest testability + future extensibility

`from_prometheus` currently does three conceptually distinct things:

1. **Normalize** the input string (ensure trailing newline).
2. **Parse** the raw text into an exposition model (via `openmetrics_parser`).
3. **Convert** each family in the exposition model into domain types.

Separating stage 3 into its own function (or making it a `TryFrom` impl for the
exposition type) means:

- Conversion logic can be tested with hand-crafted exposition values, without going
  through the text parser.
- Adding a new supported type (e.g., `Summary` in future) touches only stage 3.
- The function that does text parsing is trivially thin and almost impossible to get
  wrong.

Sketch:

```rust
impl TryFrom<openmetrics_parser::PrometheusExposition<'_>> for MetricCollection {
    type Error = PrometheusDeserializationError;

    fn try_from(
        (exposition, now): (openmetrics_parser::PrometheusExposition<'_>, DurationSinceUnixEpoch),
    ) -> Result<Self, Self::Error> {
        // family-iteration logic (proposal 1 applies here)
    }
}

impl PrometheusDeserializable for MetricCollection {
    fn from_prometheus(input: &str, now: DurationSinceUnixEpoch) -> Result<Self, PrometheusDeserializationError> {
        let input = ensure_trailing_newline(input);
        let exposition = openmetrics_parser::prometheus::parse_prometheus(&input)
            .map_err(|e| PrometheusDeserializationError::ParseError { message: e.to_string() })?;
        MetricCollection::try_from((exposition, now))
    }
}
```

Note: `TryFrom` with a tuple is a workaround for the `now` context parameter, which
is not ideal. An alternative is a newtype `ParsedExposition(exposition, now)`.

---

## Summary table

| #   | Proposal                                                          | Effort | Impact                    |
| --- | ----------------------------------------------------------------- | ------ | ------------------------- |
| 1   | Extract generic `parse_family_samples` helper                     | Low    | High                      |
| 2   | Name float guard as `is_whole_u64_representable`                  | Low    | Medium                    |
| 3   | Extract `description_from_help`                                   | Low    | Low–Medium                |
| 4   | Use `Cow<str>` for input normalization                            | Low    | Readability               |
| 5   | Return `Option` from `parse_prometheus_timestamp`                 | Low    | Readability + testability |
| 6   | Use `TryFrom` for `Counter`/`Gauge` extraction                    | Medium | Idiomatic                 |
| 7   | Implement `From` conversions instead of `collection_error` helper | Low    | Small                     |
| 8   | Decompose into normalize → parse → convert pipeline               | High   | Highest testability       |
