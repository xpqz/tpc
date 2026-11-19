program WithFieldHidesFunction;
type
  PRec = ^Rec;
  Rec = record
    Size: Integer;
    Name: string[8];
    Table: array[1..3] of Integer;
    Next: PRec;
  end;
var
  r, s: Rec;

function Size: Integer;
begin
  Size := -1
end;

function Name(i: Integer): Integer;
begin
  Name := i + 1000
end;

function Table(i: Integer): Integer;
begin
  Table := -i
end;

procedure Show(n: Integer);
begin
  WriteLn('show ', n)
end;

begin
  r.Size := 5;
  r.Name := 'field';
  r.Table[1] := 10;
  r.Table[2] := 20;
  r.Table[3] := 30;
  s.Size := 7;
  r.Next := @s;
  WriteLn(Size, ' ', Name(1), ' ', Table(2));
  with r do
  begin
    WriteLn(Size);
    WriteLn(Size + 1);
    WriteLn(Size * 2 - Size);
    WriteLn(Name);
    WriteLn(Length(Name));
    WriteLn(Table[2]);
    WriteLn(Table[Size - 3]);
    WriteLn(Next^.Size);
    Show(Size);
    if Size = 5 then WriteLn('field wins');
    Size := Size + 1;
    WriteLn(Size)
  end;
  with s do
    WriteLn(Size);
  with r, s do
    WriteLn(Size);
  WriteLn(Size, ' ', Name(2))
end.
