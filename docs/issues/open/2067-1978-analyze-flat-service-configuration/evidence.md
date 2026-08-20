# Evidence Ledger: Flat Heterogeneous Service Configuration

> **Status:** Planned
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Decision record:** [analysis.md](analysis.md)

This ledger holds reproducible evidence for the analysis. A record may cite source code, a
test-only prototype, an exact command, or a manual review. It must not claim a production schema
or runtime change was implemented.

## E1: Current-State Baseline

- **Question:** What current configuration, runtime, identity, shared-state, and redaction
  contracts constrain the analysis?
- **Status:** TODO
- **Method:** TODO
- **Observation:** TODO
- **Conclusion:** TODO
- **Report Links:** `analysis.md` sections "Current-State Baseline" and "Runtime and Normalization Model".

## E2: Configuration Representation Feasibility

- **Question:** Which TOML/Rust enum representations parse, serialize, validate, and support
  required configuration-source behavior?
- **Status:** TODO
- **Method:** TODO
- **Observation:** TODO
- **Conclusion:** TODO
- **Report Links:** `analysis.md` sections "Candidate Representations" and "Feasibility Results".

## E3: Runtime and Identity Model

- **Question:** Can one normalization model preserve role-local IDs, container lookups, startup
  dependencies, registration, and metrics behavior for interleaved services?
- **Status:** TODO
- **Method:** TODO
- **Observation:** TODO
- **Conclusion:** TODO
- **Report Links:** `analysis.md` sections "Runtime and Normalization Model" and "Identity, Ordering, and Migration".

## E4: Migration, Schema Lifecycle, and Security

- **Question:** What migration order, schema transition policy, dependency order, and redaction
  constraints would a successor schema require?
- **Status:** TODO
- **Method:** TODO
- **Observation:** TODO
- **Conclusion:** TODO
- **Report Links:** `analysis.md` sections "Identity, Ordering, and Migration" and "Schema Lifecycle, Security, and Compatibility".

## E5: Final Report Review

- **Question:** Does every material recommendation in `analysis.md` have sufficient evidence,
  and does the recommendation remain analysis-only?
- **Status:** TODO
- **Method:** TODO
- **Observation:** TODO
- **Conclusion:** TODO
- **Report Links:** `analysis.md` section "Executive Decision" and "Cost, Risks, and Recommendation".
