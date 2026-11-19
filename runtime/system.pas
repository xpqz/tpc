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

function Lo(x: Word): Byte;
begin
  Lo := x and 255
end;

function Hi(x: Word): Byte;
begin
  Hi := x shr 8
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
const HeapLimit = 983040;
type PLong = ^LongInt;
var HeapOrg, HeapPtr, FreeList: LongInt;

procedure HeapUnlink(prev, next: LongInt);
begin
  if prev = 0 then FreeList := next else PLong(prev + 4)^ := next
end;

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
    if PLong(cur)^ >= n then
    begin
      if PLong(cur)^ > n then
      begin
        PLong(cur + n)^ := PLong(cur)^ - n;
        PLong(cur + n + 4)^ := PLong(cur + 4)^;
        HeapUnlink(prev, cur + n)
      end
      else HeapUnlink(prev, PLong(cur + 4)^);
      HeapGet := Pointer(cur);
      Exit
    end;
    prev := cur;
    cur := PLong(cur + 4)^
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
    if cur >= mark then HeapUnlink(prev, PLong(cur + 4)^) else prev := cur;
    cur := PLong(cur + 4)^
  end
end;

function MemAvail: LongInt;
var cur, n: LongInt;
begin
  n := HeapLimit - HeapPtr;
  cur := FreeList;
  while cur <> 0 do
  begin
    n := n + PLong(cur)^;
    cur := PLong(cur + 4)^
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
    if PLong(cur)^ > n then n := PLong(cur)^;
    cur := PLong(cur + 4)^
  end;
  MaxAvail := n
end;
