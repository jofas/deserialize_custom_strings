deserialize_custom_strings
==========================

Utility functions for deserializing custom strings using ``serde``,
e.g. for deserializing email addresses or phone numbers.


TODO
----

* ``deserialize_u64 -> deserialize_to_type<T: FromStr>``

* make deserialization fail when format is incorrect
