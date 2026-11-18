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
