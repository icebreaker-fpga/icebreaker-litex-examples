UART_PHY
========

Register Listing for UART_PHY
-----------------------------

+------------------------------------------------------+-------------------------------------------+
| Register                                             | Address                                   |
+======================================================+===========================================+
| :ref:`UART_PHY_TUNING_WORD3 <UART_PHY_TUNING_WORD3>` | :ref:`0xe0001000 <UART_PHY_TUNING_WORD3>` |
+------------------------------------------------------+-------------------------------------------+
| :ref:`UART_PHY_TUNING_WORD2 <UART_PHY_TUNING_WORD2>` | :ref:`0xe0001004 <UART_PHY_TUNING_WORD2>` |
+------------------------------------------------------+-------------------------------------------+
| :ref:`UART_PHY_TUNING_WORD1 <UART_PHY_TUNING_WORD1>` | :ref:`0xe0001008 <UART_PHY_TUNING_WORD1>` |
+------------------------------------------------------+-------------------------------------------+
| :ref:`UART_PHY_TUNING_WORD0 <UART_PHY_TUNING_WORD0>` | :ref:`0xe000100c <UART_PHY_TUNING_WORD0>` |
+------------------------------------------------------+-------------------------------------------+

UART_PHY_TUNING_WORD3
^^^^^^^^^^^^^^^^^^^^^

`Address: 0xe0001000 + 0x0 = 0xe0001000`

    Bits 24-31 of `UART_PHY_TUNING_WORD`.

    .. wavedrom::
        :caption: UART_PHY_TUNING_WORD3

        {
            "reg": [
                {"name": "tuning_word[31:24]", "attr": 'reset: 2', "bits": 8}
            ], "config": {"hspace": 400, "bits": 8, "lanes": 1 }, "options": {"hspace": 400, "bits": 8, "lanes": 1}
        }


UART_PHY_TUNING_WORD2
^^^^^^^^^^^^^^^^^^^^^

`Address: 0xe0001000 + 0x4 = 0xe0001004`

    Bits 16-23 of `UART_PHY_TUNING_WORD`.

    .. wavedrom::
        :caption: UART_PHY_TUNING_WORD2

        {
            "reg": [
                {"name": "tuning_word[23:16]", "attr": 'reset: 117', "bits": 8}
            ], "config": {"hspace": 400, "bits": 8, "lanes": 1 }, "options": {"hspace": 400, "bits": 8, "lanes": 1}
        }


UART_PHY_TUNING_WORD1
^^^^^^^^^^^^^^^^^^^^^

`Address: 0xe0001000 + 0x8 = 0xe0001008`

    Bits 8-15 of `UART_PHY_TUNING_WORD`.

    .. wavedrom::
        :caption: UART_PHY_TUNING_WORD1

        {
            "reg": [
                {"name": "tuning_word[15:8]", "attr": 'reset: 37', "bits": 8}
            ], "config": {"hspace": 400, "bits": 8, "lanes": 1 }, "options": {"hspace": 400, "bits": 8, "lanes": 1}
        }


UART_PHY_TUNING_WORD0
^^^^^^^^^^^^^^^^^^^^^

`Address: 0xe0001000 + 0xc = 0xe000100c`

    Bits 0-7 of `UART_PHY_TUNING_WORD`.

    .. wavedrom::
        :caption: UART_PHY_TUNING_WORD0

        {
            "reg": [
                {"name": "tuning_word[7:0]", "attr": 'reset: 70', "bits": 8}
            ], "config": {"hspace": 400, "bits": 8, "lanes": 1 }, "options": {"hspace": 400, "bits": 8, "lanes": 1}
        }


