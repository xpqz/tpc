{ The System unit: the declaration part that is parsed ahead of every program, so its routines
  are visible to all of them. Only the routines a program reaches are compiled in.

  Two kinds of routine live here. Some are standard Pascal and Turbo Pascal routines (Length,
  Copy, Pos, UpCase, ...). The others are helpers that the compiler calls on its own, found by
  name after this part is checked (compiler/src/symbols.rs, `Helper`): AbsLong, RangeChk, StrCat,
  StrCmp, StrAssign, StrInt, ValLong, ValReal, StrDelete, StrInsert, SetOp, SetAdd, SetIn,
  SetCmp, HeapGet, HeapPut, HeapMark, HeapRelease, HostWrite, RtError, HostHalt, Fmt_Real, TextOut,
  TextNewLine, TextFlush, CheckIo, Assign, Eof, Eoln, ReadChar, ReadLong, ReadReal, ReadStr,
  ReadLnEnd, ExpReal, LnReal, SinReal, CosReal, ArcTanReal, RandomReal, RandomWord.
  HeapOrg, Text and Input are also found by name.

  The console hooks (ConsoleOut, ConsoleLine, ConsoleHooked) are how Crt takes over Write and
  ReadLn; they are declared with the Text files below.

  A routine declared `external` has no body: it is a WASM import from module `host`, named
  like the routine in lower case or by the string after `external`, and present in a program
  only if the program reaches it (docs/abi.md section 8). The compiler's own output code calls
  HostWrite, RtError and Fmt_Real directly; Write and WriteLn are expanded by the compiler,
  not written here. Only the runtime sources may use `external`.

  Strings are length-prefixed byte arrays, so a `var` parameter of type String accepts a string
  of any declared size and the helpers read and write the length in byte 0. A function that
  returns a String receives the address of its result as a hidden first parameter. }

{$I-}
{ System is compiled with I/O checking off: its own calls never raise a pending error; the
  compiler adds the check after the calls of a program (CheckIo). }

{ Host imports. HostWrite cannot be called Write, a predeclared name, so it names its import.
  The Fs_ routines return a handle or a count, or a negative number: the negated TP I/O error
  number. }
procedure HostWrite(fd, address, count: LongInt); external 'write';
procedure RtError(code: LongInt); external;
procedure HostHalt(code: LongInt); external 'halt';
function Fmt_Real(x: Real; width, decimals, buffer: LongInt): LongInt; external;
function Fs_Open(name, count, mode: LongInt): LongInt; external;
function Fs_Read(handle, address, count: LongInt): LongInt; external;
function Fs_Write(handle, address, count: LongInt): LongInt; external;
function Fs_Close(handle: LongInt): LongInt; external;
function ReadLine(address, max: LongInt): LongInt; external;
function HostKey: LongInt; external 'readkey';
function HostTime(what: LongInt): LongInt; external 'time';

var
  ExitCode: Integer;

{ Implicit helper: Abs calls it. }
function AbsLong(x: LongInt): LongInt;
begin
  if x < 0 then
    AbsLong := -x
  else
    AbsLong := x
end;

{ Implicit helper for range checking: v must lie in lo..hi. }
function RangeChk(v, lo, hi: LongInt): LongInt;
begin
  if (v < lo) or (v > hi) then RunError(201);
  RangeChk := v
end;

{ Hi, Lo and Swap use the low word of any integer. }
function Lo(x: LongInt): Byte;
begin
  Lo := Byte(x and 255)
end;

function Hi(x: LongInt): Byte;
begin
  Hi := Byte((x shr 8) and 255)
end;

function Swap(x: LongInt): Word;
begin
  Swap := Word(((x and 255) shl 8) or ((x shr 8) and 255))
end;

function ParamCount: Integer;
begin
  ParamCount := 0
end;

function ParamStr(n: LongInt): String;
begin
  ParamStr := ''
end;

function Pi: Real;
begin
  Pi := 3.14159265358979323846
end;

{ Random numbers: the linear congruential generator of Turbo Pascal 5.0, step for step, so that
  a program that sets RandSeed gets the very numbers TP does.
  Every draw first advances the seed, modulo 2^32:
      RandSeed := RandSeed * 134775813 + 1
  and then reads the new seed. Random(n) is the high 16 bits of the new seed, taken modulo n
  (0 when n is 0); Random without an argument is the new seed read as an unsigned number,
  divided by 2^32, a Real in [0, 1). A LongInt product wraps around in WASM, which is the
  modulo. The compiler calls RandomWord for Random(n) and RandomReal for Random. }
var
  RandSeed: LongInt;

procedure NextRandSeed;
begin
  RandSeed := RandSeed * 134775813 + 1
end;

function RandomWord(range: Word): Word;
var highBits: LongInt;
begin
  NextRandSeed;
  highBits := (RandSeed shr 16) and 65535;
  if range = 0 then
    RandomWord := 0
  else
    RandomWord := Word(highBits mod range)
end;

function RandomReal: Real;
var unsigned: Real;
begin
  NextRandSeed;
  unsigned := RandSeed;
  if RandSeed < 0 then unsigned := unsigned + 4294967296.0;
  RandomReal := unsigned / 4294967296.0
end;

{ Seeds the generator from the clock: the time of day in hundredths of a second. Reading
  time(0) first fixes the reading that time(1) and time(2) describe (docs/abi.md section 8). }
procedure Randomize;
var day: LongInt;
begin
  day := HostTime(0);
  RandSeed := HostTime(1) * 100 + HostTime(2)
end;

{ The transcendental functions. The compiler calls ExpReal, LnReal, SinReal, CosReal and
  ArcTanReal for Exp, Ln, Sin, Cos and ArcTan. Each reduces its argument to a small range,
  where a Taylor series converges to full double precision in a few terms, and then undoes
  the reduction. Their results agree with a correctly rounded value to about 1E-15 (relative)
  for arguments of moderate size. }

const
  Ln2 = 0.693147180559945309417;
  { Ln2 as a high part with trailing zero bits and a small remainder, so that K * Ln2High is
    exact for any integer K the reduction in ExpReal produces. }
  Ln2High = 6.93147180369123816490E-01;
  Ln2Low = 1.90821492927058770002E-10;
  HalfPi = 1.57079632679489661923;
  { The same split for Pi / 2, used to reduce the argument of SinReal and CosReal. }
  HalfPiHigh = 1.57079632673412561417;
  HalfPiLow = 6.07710050650619224932E-11;

{ e ** x. Write x = k * Ln2 + r with |r| <= Ln2 / 2; then e ** x = 2 ** k * e ** r, and
  e ** r is the series 1 + r + r**2 / 2! + ... }
function ExpReal(x: Real): Real;
var
  k, i: Integer;
  r, term, sum: Real;
begin
  if x > 709.78 then
    RunError(205);
  if x < -745 then
  begin
    ExpReal := 0;
    Exit
  end;
  if x >= 0 then
    k := Trunc(x / Ln2 + 0.5)
  else
    k := -Trunc(-x / Ln2 + 0.5);
  r := (x - k * Ln2High) - k * Ln2Low;
  term := 1;
  sum := 1;
  for i := 1 to 17 do
  begin
    term := term * r / i;
    sum := sum + term
  end;
  { Scaling by two is exact, so it is done one step at a time. }
  while k > 0 do
  begin
    sum := sum * 2;
    k := k - 1
  end;
  while k < 0 do
  begin
    sum := sum / 2;
    k := k + 1
  end;
  ExpReal := sum
end;

{ Natural logarithm. Scale x by powers of two into [0.7071, 1.4142), so that x = m * 2 ** e
  and ln x = e * Ln2 + ln m. With s = (m - 1) / (m + 1), |s| <= 0.1716 and
  ln m = 2 * (s + s**3 / 3 + s**5 / 5 + ...). Zero and negative arguments are error 207. }
function LnReal(x: Real): Real;
var
  e, i: Integer;
  s, s2, power, sum: Real;
begin
  if x <= 0 then
    RunError(207);
  e := 0;
  while x >= 1.41421356237309505 do
  begin
    x := x / 2;
    e := e + 1
  end;
  while x < 0.707106781186547524 do
  begin
    x := x * 2;
    e := e - 1
  end;
  s := (x - 1) / (x + 1);
  s2 := s * s;
  power := s;
  sum := 0;
  for i := 0 to 14 do
  begin
    sum := sum + power / (2 * i + 1);
    power := power * s2
  end;
  LnReal := e * Ln2 + 2 * sum
end;

{ Sine of r for |r| <= Pi / 4: r - r**3 / 3! + r**5 / 5! - ... }
function SinSeries(r: Real): Real;
var
  i: Integer;
  term, sum: Real;
begin
  term := r;
  sum := r;
  for i := 1 to 10 do
  begin
    term := -term * r * r / ((2 * i) * (2 * i + 1));
    sum := sum + term
  end;
  SinSeries := sum
end;

{ Cosine of r for |r| <= Pi / 4: 1 - r**2 / 2! + r**4 / 4! - ... }
function CosSeries(r: Real): Real;
var
  i: Integer;
  term, sum: Real;
begin
  term := 1;
  sum := 1;
  for i := 1 to 10 do
  begin
    term := -term * r * r / ((2 * i - 1) * (2 * i));
    sum := sum + term
  end;
  CosSeries := sum
end;

{ Writes x = n * Pi / 2 + r with |r| <= Pi / 4 and returns n mod 4 as the quadrant (0 to 3).
  The sign and swap rules of the quadrants are in SinReal and CosReal. Accurate while
  |x| is below about 1E9; beyond that the multiple of Pi / 2 is not exact. }
function Reduce(x: Real; var r: Real): Integer;
var
  n, whole: Real;
  quadrant: Integer;
begin
  if x >= 0 then
    n := Int(x / HalfPi + 0.5)
  else
    n := -Int(-x / HalfPi + 0.5);
  r := (x - n * HalfPiHigh) - n * HalfPiLow;
  whole := n - 4 * Int(n / 4);
  quadrant := Trunc(whole);
  if quadrant < 0 then
    quadrant := quadrant + 4;
  Reduce := quadrant
end;

function SinReal(x: Real): Real;
var
  r: Real;
begin
  case Reduce(x, r) of
    0: SinReal := SinSeries(r);
    1: SinReal := CosSeries(r);
    2: SinReal := -SinSeries(r);
  else
    SinReal := -CosSeries(r)
  end
end;

function CosReal(x: Real): Real;
var
  r: Real;
begin
  case Reduce(x, r) of
    0: CosReal := CosSeries(r);
    1: CosReal := -SinSeries(r);
    2: CosReal := -CosSeries(r);
  else
    CosReal := SinSeries(r)
  end
end;

{ Arc tangent. For |x| > 1, ArcTan x = Pi / 2 - ArcTan (1 / x). Then the identity
  ArcTan x = 2 * ArcTan (x / (1 + Sqrt (1 + x**2))) is applied three times, shrinking |x| to
  at most 0.1, where the series x - x**3 / 3 + x**5 / 5 - ... converges quickly. }
function ArcTanReal(x: Real): Real;
var
  i: Integer;
  negative, inverted: Boolean;
  x2, power, sum: Real;
begin
  negative := x < 0;
  if negative then
    x := -x;
  inverted := x > 1;
  if inverted then
    x := 1 / x;
  for i := 1 to 3 do
    x := x / (1 + Sqrt(1 + x * x));
  x2 := x * x;
  power := x;
  sum := 0;
  for i := 0 to 8 do
  begin
    sum := sum + power / (2 * i + 1);
    power := -power * x2
  end;
  sum := sum * 8;
  if inverted then
    sum := HalfPi - sum;
  if negative then
    sum := -sum;
  ArcTanReal := sum
end;

function UpCase(c: Char): Char;
begin
  if (c >= 'a') and (c <= 'z') then
    UpCase := Chr(Ord(c) - 32)
  else
    UpCase := c
end;

{ String helpers the compiler calls implicitly (bound by name after System is loaded). The var
  parameters take the addresses of any string, whatever its declared size. }

{ d := a + b, truncated to 255; d is a temporary distinct from a and b. }
procedure StrCat(var d, a, b: String);
var i, n, m: Integer;
begin
  n := Ord(a[0]);
  m := Ord(b[0]);
  if n + m > 255 then m := 255 - n;
  for i := 1 to n do d[i] := a[i];
  for i := 1 to m do d[n + i] := b[i];
  d[0] := Chr(n + m)
end;

{ -1, 0 or 1: byte-wise lexicographic order, then by length. }
function StrCmp(var a, b: String): Integer;
var i, n: Integer;
begin
  n := Ord(a[0]);
  if Ord(b[0]) < n then n := Ord(b[0]);
  StrCmp := 0;
  i := 1;
  while (i <= n) and (a[i] = b[i]) do i := i + 1;
  if i <= n then
    if a[i] < b[i] then StrCmp := -1 else StrCmp := 1
  else if Ord(a[0]) < Ord(b[0]) then StrCmp := -1
  else if Ord(a[0]) > Ord(b[0]) then StrCmp := 1
end;

{ d := s truncated to max characters. }
procedure StrAssign(var d, s: String; max: Integer);
var i, n: Integer;
begin
  n := Ord(s[0]);
  if n > max then n := max;
  for i := 1 to n do d[i] := s[i];
  d[0] := Chr(n)
end;

{ Digits are produced last first into r, then copied reversed into t after the padding.
  Str(v:w, s) of an integer is s := StrInt(v, w, max), max the capacity of s: a wider field is
  cut to it. }
function StrInt(v: LongInt; w, max: Integer): String;
var r, t: String; i, n, pad, d: Integer; neg: Boolean;
begin
  neg := v < 0;
  n := 0;
  repeat
    d := Integer(v mod 10);
    if d < 0 then d := -d;
    n := n + 1;
    r[n] := Chr(48 + d);
    v := v div 10
  until v = 0;
  if neg then
  begin
    n := n + 1;
    r[n] := '-'
  end;
  if w > max then w := max;
  pad := w - n;
  if pad < 0 then pad := 0;
  for i := 1 to pad do t[i] := ' ';
  for i := 1 to n do t[pad + i] := r[n + 1 - i];
  t[0] := Chr(pad + n);
  StrInt := t
end;

{ Val(s, v, code) of an integer is v := ValLong(s, code): optional spaces, a sign, then decimal
  digits or $ and hex digits. code is the position of the first bad character or of the digit that
  overflows LongInt (the sign counts, and the magnitude is limited, so -2147483648 fails), else 0;
  the value is 0 on error. }
function ValLong(s: String; var code: Integer): LongInt;
var i, n, d: Integer; v: LongInt; neg, hex: Boolean; c: Char;
begin
  n := Ord(s[0]);
  i := 1;
  v := 0;
  neg := False;
  hex := False;
  code := 0;
  while (i <= n) and (s[i] = ' ') do i := i + 1;
  if (i <= n) and ((s[i] = '-') or (s[i] = '+')) then
  begin
    neg := s[i] = '-';
    i := i + 1
  end;
  if (i <= n) and (s[i] = '$') then
  begin
    hex := True;
    i := i + 1
  end;
  if i > n then code := i;
  while (i <= n) and (code = 0) do
  begin
    c := s[i];
    d := -1;
    if (c >= '0') and (c <= '9') then d := Ord(c) - 48
    else if hex and (c >= 'A') and (c <= 'F') then d := Ord(c) - 55
    else if hex and (c >= 'a') and (c <= 'f') then d := Ord(c) - 87;
    if d < 0 then code := i
    else if hex then
    begin
      if ((v shr 28) and 15) <> 0 then code := i else v := (v shl 4) + d
    end
    else if v > (2147483647 - d) div 10 then code := i
    else v := v * 10 + d;
    i := i + 1
  end;
  if code <> 0 then ValLong := 0
  else if neg then ValLong := -v
  else ValLong := v
end;

{ Val(s, r, code) of a Real is r := ValReal(s, code): sign, digits, optional .digits and
  E[+-]digits. The value is mantissa * 10^exponent by one multiplication or division. }
function ValReal(s: String; var code: Integer): Real;
var i, n, ex, e10, k: Integer; m, p: Real; neg, eneg: Boolean; c: Char;
begin
  n := Ord(s[0]);
  i := 1;
  m := 0;
  ex := 0;
  e10 := 0;
  neg := False;
  eneg := False;
  code := 0;
  while (i <= n) and (s[i] = ' ') do i := i + 1;
  if (i <= n) and ((s[i] = '-') or (s[i] = '+')) then
  begin
    neg := s[i] = '-';
    i := i + 1
  end;
  if (i > n) or not (s[i] in ['0'..'9', '.']) then code := i;
  while (code = 0) and (i <= n) and (s[i] in ['0'..'9']) do
  begin
    m := m * 10 + (Ord(s[i]) - 48);
    i := i + 1
  end;
  if (code = 0) and (i <= n) and (s[i] = '.') then
  begin
    i := i + 1;
    while (i <= n) and (s[i] in ['0'..'9']) do
    begin
      m := m * 10 + (Ord(s[i]) - 48);
      e10 := e10 - 1;
      i := i + 1
    end
  end;
  if (code = 0) and (i <= n) and ((s[i] = 'E') or (s[i] = 'e')) then
  begin
    i := i + 1;
    if (i <= n) and ((s[i] = '-') or (s[i] = '+')) then
    begin
      eneg := s[i] = '-';
      i := i + 1
    end;
    if (i > n) or not (s[i] in ['0'..'9']) then code := i;
    while (code = 0) and (i <= n) and (s[i] in ['0'..'9']) do
    begin
      if ex < 1000 then ex := ex * 10 + Ord(s[i]) - 48;
      i := i + 1
    end;
    if eneg then ex := -ex;
    e10 := e10 + ex
  end;
  if (code = 0) and (i <= n) then code := i;
  if code <> 0 then ValReal := 0
  else
  begin
    p := 1;
    for k := 1 to Abs(e10) do p := p * 10;
    if neg then m := -m;
    if e10 < 0 then ValReal := m / p else ValReal := m * p
  end
end;

function Length(s: String): Integer;
begin
  Length := Ord(s[0])
end;

function Pos(sub, s: String): Integer;
var i, j: Integer;
begin
  Pos := 0;
  for i := 1 to Ord(s[0]) - Ord(sub[0]) + 1 do
  begin
    j := 1;
    while (j <= Ord(sub[0])) and (s[i + j - 1] = sub[j]) do j := j + 1;
    if j > Ord(sub[0]) then
    begin
      Pos := i;
      Exit
    end
  end
end;

function Copy(s: String; i, n: Integer): String;
var r: String; k: Integer;
begin
  if i < 1 then i := 1;
  if n > Ord(s[0]) - i + 1 then n := Ord(s[0]) - i + 1;
  if n < 0 then n := 0;
  for k := 1 to n do r[k] := s[i + k - 1];
  r[0] := Chr(n);
  Copy := r
end;

{ Delete(s, i, n) is s := StrDelete(s, i, n). A position of 0 or less starts at 1 and deletes
  n + i - 1 characters, as TP does. }
function StrDelete(s: String; i, n: Integer): String;
begin
  if i <= 0 then
  begin
    n := n + i - 1;
    i := 1
  end;
  if (n <= 0) or (i > Ord(s[0])) then StrDelete := s
  else StrDelete := Copy(s, 1, i - 1) + Copy(s, i + n, 255)
end;

{ Insert(src, s, i) is s := StrInsert(src, s, i); the caller truncates to the size of s. }
function StrInsert(src, s: String; i: Integer): String;
begin
  if i < 1 then i := 1;
  StrInsert := Copy(s, 1, i - 1) + src + Copy(s, i, 255)
end;

{ Set helpers the compiler calls: a set is 32 bytes, bit i of byte i div 8 is element i. }
type SetBuf = array[0..31] of Byte;

{ d := a + b, a * b or a - b for op 0, 1, 2; d may be a or b. }
procedure SetOp(var d, a, b: SetBuf; op: Integer);
var i: Integer;
begin
  for i := 0 to 31 do
    if op = 0 then d[i] := a[i] or b[i]
    else if op = 1 then d[i] := a[i] and b[i]
    else d[i] := a[i] and not b[i]
end;

{ d := d + [lo..hi]; elements are taken modulo 256, as TP does. }
procedure SetAdd(var d: SetBuf; lo, hi: Integer);
var i: Integer;
begin
  hi := hi and 255;
  for i := lo and 255 to hi do d[i shr 3] := d[i shr 3] or (1 shl (i and 7))
end;

function SetIn(var s: SetBuf; i: Integer): Boolean;
begin
  SetIn := (i >= 0) and (i <= 255) and ((s[i shr 3] and (1 shl (i and 7))) <> 0)
end;

{ Bit 0: a has an element that b lacks. Bit 1: b has an element that a lacks. }
function SetCmp(var a, b: SetBuf): Integer;
var i, r: Integer;
begin
  r := 0;
  for i := 0 to 31 do
  begin
    if (a[i] and not b[i]) <> 0 then r := r or 1;
    if (b[i] and not a[i]) <> 0 then r := r or 2
  end;
  SetCmp := r
end;

{ Heap (decision 18): from HeapOrg up to the stack limit, 8-byte blocks. HeapOrg and HeapPtr are
  adjacent and both set to the end of static data by the compiler. A free block holds its size
  and the address of the next free block; the list is first-fit and unordered. }
const HeapLimit = 983040; { the 64 KB below the top of memory belong to the stack }
type PLong = ^LongInt;
var HeapOrg, HeapPtr, FreeList: LongInt;

function BlockSize(block: LongInt): LongInt;
begin
  BlockSize := PLong(block)^
end;

function NextFree(block: LongInt): LongInt;
begin
  NextFree := PLong(block + 4)^
end;

{ Removes the free block that follows `prev` (the list head when prev is 0), whose successor
  is `next`. }
procedure HeapUnlink(prev, next: LongInt);
begin
  if prev = 0 then FreeList := next else PLong(prev + 4)^ := next
end;

{ Blocks are a multiple of 8 bytes, at least 8: (n + 7) and -8 rounds up. }
function HeapSize(n: LongInt): LongInt;
begin
  n := (n + 7) and -8;
  if n = 0 then n := 8;
  HeapSize := n
end;

{ New and GetMem. }
function HeapGet(n: LongInt): Pointer;
var cur, prev: LongInt;
begin
  n := HeapSize(n);
  prev := 0;
  cur := FreeList;
  while cur <> 0 do
  begin
    if BlockSize(cur) >= n then
    begin
      if BlockSize(cur) > n then
      begin
        PLong(cur + n)^ := BlockSize(cur) - n;
        PLong(cur + n + 4)^ := NextFree(cur);
        HeapUnlink(prev, cur + n)
      end
      else HeapUnlink(prev, NextFree(cur));
      HeapGet := Pointer(cur);
      Exit
    end;
    prev := cur;
    cur := NextFree(cur)
  end;
  if HeapLimit - HeapPtr < n then RunError(203);
  HeapGet := Pointer(HeapPtr);
  HeapPtr := HeapPtr + n
end;

{ Dispose and FreeMem: nil, addresses outside the used heap and misaligned ones are error 204. }
procedure HeapPut(p: Pointer; n: LongInt);
var a: LongInt;
begin
  a := LongInt(p);
  n := HeapSize(n);
  if (a < HeapOrg) or (a + n > HeapPtr) or (((a - HeapOrg) and 7) <> 0) then RunError(204);
  PLong(a)^ := n;
  PLong(a + 4)^ := FreeList;
  FreeList := a
end;

function HeapMark: Pointer;
begin
  HeapMark := Pointer(HeapPtr)
end;

{ Resets the bump pointer and drops the free blocks above the mark. }
procedure HeapRelease(p: Pointer);
var mark, cur, prev: LongInt;
begin
  mark := LongInt(p);
  HeapPtr := mark;
  prev := 0;
  cur := FreeList;
  while cur <> 0 do
  begin
    if cur >= mark then HeapUnlink(prev, NextFree(cur)) else prev := cur;
    cur := NextFree(cur)
  end
end;

function MemAvail: LongInt;
var cur, n: LongInt;
begin
  n := HeapLimit - HeapPtr;
  cur := FreeList;
  while cur <> 0 do
  begin
    n := n + BlockSize(cur);
    cur := NextFree(cur)
  end;
  MemAvail := n
end;

function MaxAvail: LongInt;
var cur, n: LongInt;
begin
  n := HeapLimit - HeapPtr;
  cur := FreeList;
  while cur <> 0 do
  begin
    if BlockSize(cur) > n then n := BlockSize(cur);
    cur := NextFree(cur)
  end;
  MaxAvail := n
end;

{ Text files. A Text is a record: the host handle, what the file is open for, a buffer of the
  bytes read and not yet used (Pos to Len) and the name given by Assign. Writing is not
  buffered: a file's output goes to the host at once, and so does the console's, unless Crt
  hooks the console (see TextFlush). Input and Output are typed
  constants naming only the first fields; their buffer is zero. The compiler writes to the
  console by passing the file address 0 to TextOut.

  An error is a code in InOutRes. While one is pending, Read, Write, Eof and Eoln do nothing
  (the items of a Read become 0, '' or ^Z, the Eofs False) and Reset, Rewrite, Append and
  Close still run and replace the code when they fail. IOResult reads and clears it. With
  checking on, the compiler calls CheckIo after each call of a program that has a Text as first
  parameter, except Assign, and after each Write. }
const
  ModeUnassigned = 0;
  ModeClosed = 1;
  ModeInput = 2;
  ModeOutput = 3;
  ModeConsoleInput = 4;
  ModeConsoleOutput = 5;
  Backspace = 8;
  CarriageReturn = 13;
  LineFeed = 10;
  Escape = 27;
  Blank = 32;
  EndOfFile = 26;
  BufferSize = 256;
  { TP I/O error numbers, as IOResult reports them. }
  IoNotAssigned = 102;
  IoNotOpen = 103;
  IoNotInput = 104;
  IoNotOutput = 105;
  IoInvalidNumber = 106;
  { The modes of Fs_Open (docs/abi.md): read, create or truncate, append; and the console's
    file descriptor. }
  HostRead = 0;
  HostCreate = 1;
  HostAppend = 2;
  StdOut = 1;
  ConsoleLineMax = 254; { room in the buffer for the CR LF that end a console line }
  NoLineInput = -2; { what `readline` returns where the host has no stdin: the browser }

type
  Text = record
    Handle: LongInt;
    Mode, Pos, Len: Integer;
    Ended: Boolean;
    Name: String[79];
    Buffer: array[0..BufferSize - 1] of Char;
  end;
  PText = ^Text;

const
  Input: Text = (Handle: 0; Mode: ModeConsoleInput);
  Output: Text = (Handle: 0; Mode: ModeConsoleOutput);

var InOutRes: Integer;

{ The console is the host's by default. Crt takes it over by setting ConsoleHooked and the two
  hooks in its initialisation: ConsoleOut receives the bytes written to the console (a line end
  is CR LF), ConsoleLine reads a console line like ReadLine does. A procedural variable cannot
  be tested for nil, so the flag says whether the hooks are set. }
type
  ConsoleWriter = procedure(address, count: LongInt);
  ConsoleLineReader = function(address, max: LongInt): LongInt;

var
  ConsoleHooked: Boolean;
  ConsoleOut: ConsoleWriter;
  ConsoleLine: ConsoleLineReader;

function IOResult: Integer;
begin
  IOResult := InOutRes;
  InOutRes := 0
end;

{ The runtime error for the pending code. }
procedure CheckIo;
var code: Integer;
begin
  if InOutRes <> 0 then
  begin
    code := InOutRes;
    InOutRes := 0;
    RunError(code)
  end
end;

{ Under Crt the console's output is held back until the end of the Write or WriteLn
  statement, as TP holds it in the file's buffer (128 bytes, flushed when full): the arguments
  of the statement are evaluated, WhereX among them, before any of its text reaches the
  screen. A statement that writes more than 128 bytes shows its first part early, as in TP.
  The compiler calls TextFlush at the end of every Write and WriteLn. }
const ConsoleBufferSize = 128;

type
  { A view of the bytes of one piece of output, of any length up to 32 KB. }
  OutputBytes = array[0..32767] of Char;
  POutputBytes = ^OutputBytes;

var
  ConsolePending: array[0..ConsoleBufferSize - 1] of Char;
  ConsolePendingCount: Integer;

procedure TextFlush;
begin
  if ConsolePendingCount > 0 then
  begin
    ConsoleOut(LongInt(@ConsolePending[0]), ConsolePendingCount);
    ConsolePendingCount := 0
  end
end;

procedure ConsoleWrite(address, count: LongInt);
var i: LongInt;
begin
  if not ConsoleHooked then HostWrite(StdOut, address, count) { the host's console }
  else { Crt's screen: collect until TextFlush }
    for i := 0 to count - 1 do
    begin
      if ConsolePendingCount = ConsoleBufferSize then TextFlush;
      ConsolePending[ConsolePendingCount] := POutputBytes(address)^[i];
      ConsolePendingCount := ConsolePendingCount + 1
    end
end;

{ Writes count bytes at address to the Text at address target, or to the console when target
  is 0. }
procedure TextOut(target, address, count: LongInt);
var f: PText; written: LongInt;
begin
  if InOutRes <> 0 then Exit;
  if target = 0 then ConsoleWrite(address, count)
  else
  begin
    f := PText(target);
    if f^.Mode = ModeConsoleOutput then ConsoleWrite(address, count)
    else if f^.Mode <> ModeOutput then InOutRes := IoNotOutput
    else
    begin
      written := Fs_Write(f^.Handle, address, count);
      if written < 0 then InOutRes := Integer(-written)
    end
  end
end;

{ The end of a line: LF on the host's console, CR LF in a file and on Crt's screen, as TP
  writes them. }
const LineEnd: array[0..1] of Char = (#13, #10);

procedure TextNewLine(target: LongInt);
begin
  if ((target = 0) or (PText(target)^.Mode = ModeConsoleOutput)) and not ConsoleHooked then
    TextOut(target, LongInt(@LineEnd[1]), 1)
  else
    TextOut(target, LongInt(@LineEnd[0]), 2)
end;

procedure Assign(var f: Text; name: String);
begin
  f.Handle := 0;
  f.Mode := ModeClosed;
  f.Pos := 0;
  f.Len := 0;
  f.Ended := False;
  f.Name := name
end;

{ A file assigned the empty name is the console: opened for input it reads standard input (or
  Crt's keyboard line), opened for output it writes standard output (or Crt's screen), the way
  Input and Output do. There is no host handle, so Close has nothing to release. }
procedure OpenConsole(var f: Text; textMode: Integer);
begin
  f.Mode := textMode;
  f.Pos := 0;
  f.Len := 0;
  f.Ended := False
end;

{ Reset, Rewrite and Append: open the assigned name with the host mode, for reading or writing.
  A file that is open is closed first; a failure leaves it closed. }
procedure OpenFile(var f: Text; hostMode, textMode: Integer);
var handle: LongInt;
begin
  if f.Mode = ModeUnassigned then
  begin
    InOutRes := IoNotAssigned;
    Exit
  end;
  if f.Name[0] = Chr(0) then
  begin
    if textMode = ModeInput then OpenConsole(f, ModeConsoleInput)
    else OpenConsole(f, ModeConsoleOutput);
    Exit
  end;
  if f.Mode >= ModeConsoleInput then Exit;
  { The status of closing the old file is not reported; `handle` takes it only because a
    function result cannot be dropped. }
  if (f.Mode = ModeInput) or (f.Mode = ModeOutput) then handle := Fs_Close(f.Handle);
  f.Mode := ModeClosed;
  handle := Fs_Open(LongInt(@f.Name[1]), Ord(f.Name[0]), hostMode);
  if handle < 0 then InOutRes := Integer(-handle)
  else
  begin
    f.Handle := handle;
    f.Mode := textMode;
    f.Pos := 0;
    f.Len := 0;
    f.Ended := False
  end
end;

procedure Reset(var f: Text);
begin
  OpenFile(f, HostRead, ModeInput)
end;

procedure Rewrite(var f: Text);
begin
  OpenFile(f, HostCreate, ModeOutput)
end;

procedure Append(var f: Text);
begin
  OpenFile(f, HostAppend, ModeOutput)
end;

procedure Close(var f: Text);
var status: LongInt;
begin
  if (f.Mode = ModeInput) or (f.Mode = ModeOutput) then
  begin
    status := Fs_Close(f.Handle);
    f.Mode := ModeClosed;
    if status < 0 then InOutRes := Integer(-status)
  end
  else if f.Mode < ModeConsoleInput then InOutRes := IoNotOpen
end;

procedure EchoKey(ch: Char);
begin
  ConsoleWrite(LongInt(@ch), 1)
end;

{ The line editor of a host without stdin (the browser, decision 46; Crt's EditLine does the
  same with its own screen): reads keys until Enter, echoing them on the console, and stores
  the line at `address`, at most `max` characters. Backspace erases the last character, Esc
  the whole line; any other control key, an extended key (0, then its scan code) and any key
  beyond `max` are ignored. Returns the length of the line. }
function EditConsoleLine(address, max: LongInt): LongInt;
var key: Char; count: LongInt; entered: Boolean;

  procedure EraseLast;
  begin
    count := count - 1;
    EchoKey(Chr(Backspace));
    EchoKey(Chr(Blank));
    EchoKey(Chr(Backspace))
  end;

begin
  count := 0;
  entered := False;
  repeat
    key := Chr(HostKey);
    if Ord(key) = 0 then key := Chr(HostKey)
    else if Ord(key) = CarriageReturn then entered := True
    else if Ord(key) = Backspace then
    begin
      if count > 0 then EraseLast
    end
    else if Ord(key) = Escape then
    begin
      while count > 0 do EraseLast
    end
    else if (Ord(key) >= Blank) and (count < max) then
    begin
      POutputBytes(address)^[count] := key;
      count := count + 1;
      EchoKey(key)
    end
  until entered;
  EchoKey(Chr(CarriageReturn));
  EchoKey(Chr(LineFeed));
  EditConsoleLine := count
end;

{ The line of a console read: Crt's hook, else the host's `readline`, else the line editor. }
function ReadConsoleLine(address, max: LongInt): LongInt;
var count: LongInt;
begin
  if ConsoleHooked then ReadConsoleLine := ConsoleLine(address, max)
  else
  begin
    count := ReadLine(address, max);
    if count = NoLineInput then count := EditConsoleLine(address, max);
    ReadConsoleLine := count
  end
end;

{ The buffer of a console file holds one line and the CR LF the host does not deliver. }
procedure FillFromConsole(var f: Text);
var count: LongInt;
begin
  count := ReadConsoleLine(LongInt(@f.Buffer[0]), ConsoleLineMax);
  if count < 0 then f.Ended := True
  else
  begin
    f.Buffer[count] := Chr(CarriageReturn);
    f.Buffer[count + 1] := Chr(LineFeed);
    f.Len := Integer(count) + 2
  end
end;

{ A ^Z byte ends the data of a text file; the bytes after it are never seen. }
procedure FillFromFile(var f: Text);
var count: LongInt; i: Integer;
begin
  count := Fs_Read(f.Handle, LongInt(@f.Buffer[0]), BufferSize);
  if count <= 0 then
  begin
    f.Ended := True;
    if count < 0 then InOutRes := Integer(-count)
  end
  else
  begin
    f.Len := Integer(count);
    i := 0;
    while (i < f.Len) and (f.Buffer[i] <> Chr(EndOfFile)) do i := i + 1;
    if i < f.Len then
    begin
      f.Len := i;
      f.Ended := True
    end
  end
end;

{ True when an unread byte is in the buffer; an empty buffer is refilled first. Fill has that
  side effect, so the readers below always put it first in an `and`: Boolean expressions stop
  at the first False operand. }
function Fill(var f: Text): Boolean;
begin
  if f.Pos >= f.Len then
  begin
    f.Pos := 0;
    f.Len := 0;
    if not f.Ended then
    begin
      if f.Mode = ModeConsoleInput then FillFromConsole(f) else FillFromFile(f)
    end
  end;
  Fill := f.Pos < f.Len
end;

{ True when f can be read now: no error is pending and f is open for input. Otherwise the
  wrong mode is error 104. }
function CanRead(var f: Text): Boolean;
begin
  CanRead := False;
  if InOutRes <> 0 then Exit;
  if (f.Mode = ModeInput) or (f.Mode = ModeConsoleInput) then CanRead := True
  else InOutRes := IoNotInput
end;

function Eof(var f: Text): Boolean;
begin
  Eof := False;
  if CanRead(f) then Eof := not Fill(f)
end;

{ The end of a line is the CR; the end of the data counts as one. }
function Eoln(var f: Text): Boolean;
begin
  Eoln := False;
  if CanRead(f) then
    if Fill(f) then Eoln := f.Buffer[f.Pos] = Chr(CarriageReturn)
    else Eoln := True
end;

function ReadChar(var f: Text): Char;
begin
  ReadChar := Chr(EndOfFile);
  if CanRead(f) and Fill(f) then
  begin
    ReadChar := f.Buffer[f.Pos];
    f.Pos := f.Pos + 1
  end
end;

{ Read of a String: the rest of the line, at most max characters. }
function ReadStr(var f: Text; max: Integer): String;
var s: String; n: Integer;
begin
  n := 0;
  if CanRead(f) then
    while (n < max) and Fill(f) and (f.Buffer[f.Pos] <> Chr(CarriageReturn)) do
    begin
      n := n + 1;
      s[n] := f.Buffer[f.Pos];
      f.Pos := f.Pos + 1
    end;
  s[0] := Chr(n);
  ReadStr := s
end;

{ The next number as text: blanks and line ends are skipped, then every byte up to the next
  one. Empty at the end of the data. }
function ReadToken(var f: Text): String;
var s: String; n: Integer;
begin
  n := 0;
  if CanRead(f) then
  begin
    while Fill(f) and (f.Buffer[f.Pos] in [' ', #9, #10, #13]) do f.Pos := f.Pos + 1;
    while Fill(f) and not (f.Buffer[f.Pos] in [' ', #9, #10, #13]) do
    begin
      if n < 255 then
      begin
        n := n + 1;
        s[n] := f.Buffer[f.Pos]
      end;
      f.Pos := f.Pos + 1
    end
  end;
  s[0] := Chr(n);
  ReadToken := s
end;

{ Read of an integer: 0 at the end of the data, error 106 for a token that is not a number. }
function ReadLong(var f: Text): LongInt;
var token: String; value: LongInt; code: Integer;
begin
  ReadLong := 0;
  token := ReadToken(f);
  if Length(token) > 0 then
  begin
    value := ValLong(token, code);
    if code <> 0 then InOutRes := IoInvalidNumber else ReadLong := value
  end
end;

function ReadReal(var f: Text): Real;
var token: String; value: Real; code: Integer;
begin
  ReadReal := 0;
  token := ReadToken(f);
  if Length(token) > 0 then
  begin
    value := ValReal(token, code);
    if code <> 0 then InOutRes := IoInvalidNumber else ReadReal := value
  end
end;

{ The end of ReadLn: everything up to and including the next CR and a LF that follows it. }
procedure ReadLnEnd(var f: Text);
begin
  if not CanRead(f) then Exit;
  while Fill(f) and (f.Buffer[f.Pos] <> Chr(CarriageReturn)) do f.Pos := f.Pos + 1;
  if Fill(f) then
  begin
    f.Pos := f.Pos + 1;
    if Fill(f) and (f.Buffer[f.Pos] = Chr(LineFeed)) then f.Pos := f.Pos + 1
  end
end;
