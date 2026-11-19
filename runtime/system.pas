{ The System unit: the declaration part that is parsed ahead of every program, so its routines
  are visible to all of them. Only the routines a program reaches are compiled in.

  Two kinds of routine live here. Some are standard Pascal and Turbo Pascal routines (Length,
  Copy, Pos, UpCase, ...). The others are helpers that the compiler calls on its own, found by
  name after this part is checked (compiler/src/check.rs, `Helper`): AbsLong, RangeChk, StrCat,
  StrCmp, StrAssign, StrInt, ValLong, ValReal, StrDelete, StrInsert, SetOp, SetAdd, SetIn,
  SetCmp, HeapGet, HeapPut, HeapMark, HeapRelease. HeapOrg is also found by name.

  Strings are length-prefixed byte arrays, so a `var` parameter of type String accepts a string
  of any declared size and the helpers read and write the length in byte 0. A function that
  returns a String receives the address of its result as a hidden first parameter. }

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
