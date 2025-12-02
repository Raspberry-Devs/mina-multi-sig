import re
import ast

# --------------------------
# Helpers
# --------------------------

def parse_bool(token):
    val = token.strip().lower() == "true"
    return [1 if val else 0, 1]   # bit width = 1

def parse_u32(token):
    return [int(token), 32]

def parse_u64(token):
    return [int(token), 64]

def parse_bytes(token):
    # token is something like: [0, 0, 0, 0, 0, 0]
    arr = ast.literal_eval(token)
    bit_width = len(arr) * 8
    # interpret bytes as big endian integer
    value = 0
    for b in arr:
        value = (value << 8) | b
    return [value, bit_width]


# --------------------------
# Main parser for format A
# --------------------------

def parse_format_A(text):
    """
    Converts:
      BOOL { val: true }
      U32 { val: 3 }
      U64 { val: 18446744073709551615 }
      BYTES { val: [0,0,0,0] }
    into normalized form: [value:int, bit_width:int]
    """

    pattern = re.compile(r"""
        (?P<type>BOOL|U32|U64|BYTES)      # the type
        \s*\{\s*val:\s*
        (?P<val>
            true|false|                   # booleans
            \d+|                          # integers
            \[[^\]]*\]                    # byte array
        )
        \s*\}
    """, re.IGNORECASE | re.VERBOSE)

    result = []

    for m in pattern.finditer(text):
        t = m.group("type").upper()
        v = m.group("val")

        if t == "BOOL":
            result.append(parse_bool(v))
        elif t == "U32":
            result.append(parse_u32(v))
        elif t == "U64":
            result.append(parse_u64(v))
        elif t == "BYTES":
            result.append(parse_bytes(v))
        else:
            raise ValueError(f"Unknown type: {t}")

    return result


# --------------------------
# Comparator
# --------------------------

def compare_arrays(arr1, arr2):
    """
    Compares element-by-element.
    arr1 is converted format A.
    arr2 is the provided second array (converted to normal python ints).
    """
    mismatches = []

    if len(arr1) != len(arr2):
        print(f"Length mismatch: {len(arr1)} vs {len(arr2)}")

    L = min(len(arr1), len(arr2))

    for i in range(L):
        a = arr1[i]
        b = arr2[i]

        # convert BigInt format like 123n into plain int if necessary
        b0 = int(str(b[0]).rstrip("n")) if isinstance(b[0], (str,)) else int(b[0])
        b1 = int(b[1])

        if a != [b0, b1]:
            mismatches.append((i, a, [b0, b1]))

    return mismatches


# --------------------------
# Example Use
# --------------------------

# (1) Paste your giant text blob here:
textA = "BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }U32 { val: 3 }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: false }BYTES { val: [0, 0, 0, 0, 0, 0] }BOOL { val: true }U64 { val: 3728633945706664709 }U32 { val: 12282 }U64 { val: 1 }U32 { val: 13092938 }U64 { val: 1133034378618331073 }BOOL { val: false }U64 { val: 54521 }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: true }U32 { val: 4294967295 }U32 { val: 3 }BOOL { val: true }U32 { val: 1 }U32 { val: 1377887 }BOOL { val: true }U64 { val: 1 }U64 { val: 18446744073709551615 }BOOL { val: true }U32 { val: 1378969044 }U32 { val: 0 }BOOL { val: true }BOOL { val: true }U64 { val: 30129300470 }U64 { val: 2835200861991394646 }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }U32 { val: 857183953 }U32 { val: 14 }BOOL { val: false }BOOL { val: true }U64 { val: 492790850673 }U64 { val: 88380204 }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: false }U32 { val: 0 }U32 { val: 4294967295 }BOOL { val: false }U64 { val: 0 }U64 { val: 18446744073709551615 }BOOL { val: false }U32 { val: 0 }U32 { val: 4294967295 }BOOL { val: false }BOOL { val: false }BOOL { val: false }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: true }BOOL { val: false }BOOL { val: true }BOOL { val: false }BOOL { val: false }BOOL { val: false }BOOL { val: false }BOOL { val: false }U32 { val: 0 }U32 { val: 4294967295 }BOOL { val: false }BOOL { val: false }BOOL { val: true }BOOL { val: false }BOOL { val: false }BOOL { val: false }"
# (2) Paste your second array here (must be valid Python literal with n removed or converted)
#     Example: replace 123n → 123
arrayB = [
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    3,
    32,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    48,
  ],
  [
    1,
    1,
  ],
  [
    3728633945706664709,
    64,
  ],
  [
    12282,
    32,
  ],
  [
    1,
    64,
  ],
  [
    13092938,
    32,
  ],
  [
    1133034378618331073,
    64,
  ],
  [
    0,
    1,
  ],
  [
    54521,
    64,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    4294967295,
    32,
  ],
  [
    3,
    32,
  ],
  [
    1,
    1,
  ],
  [
    1,
    32,
  ],
  [
    1377887,
    32,
  ],
  [
    1,
    1,
  ],
  [
    1,
    64,
  ],
  [
    18446744073709551615,
    64,
  ],
  [
    1,
    1,
  ],
  [
    1378969044,
    32,
  ],
  [
    0,
    32,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    30129300470,
    64,
  ],
  [
    2835200861991394646,
    64,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    857183953,
    32,
  ],
  [
    14,
    32,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    492790850673,
    64,
  ],
  [
    88380204,
    64,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    32,
  ],
  [
    4294967295,
    32,
  ],
  [
    0,
    1,
  ],
  [
    0,
    64,
  ],
  [
    18446744073709551615,
    64,
  ],
  [
    0,
    1,
  ],
  [
    0,
    32,
  ],
  [
    4294967295,
    32,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    32,
  ],
  [
    4294967295,
    32,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    1,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
  [
    0,
    1,
  ],
]
# Convert first format → normalized
convertedA = parse_format_A(textA)

# Compare
mismatches = compare_arrays(convertedA, arrayB)

if not mismatches:
    print("✓ Arrays match exactly.")
else:
    print("✗ Mismatches found:")
    for idx, a, b in mismatches:
        print(f"Index {idx}:\n  A={a}\n  B={b}")
