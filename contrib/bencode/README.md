# Bencode
This library allows for the creation and parsing of bencode encodings.

Bencode is the binary encoding used throughout bittorrent technologies from metainfo files to DHT messages. Bencode types include integers, byte arrays, lists, and dictionaries, of which the last two can hold any bencode type (they could be recursively constructed).