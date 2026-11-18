program RecordFieldsOfEveryKind;
type Color = (Red, Green, Blue); Small = 1..9;
     Mixed = record
       b: Byte; s: ShortInt; i: Integer; w: Word; l: LongInt;
       flag: Boolean; ch: Char; c: Color; sm: Small
     end;
var m: Mixed; copy: Mixed; arr: array[1..3] of Mixed;
begin
  m.b := 250; m.s := -100; m.i := -30000; m.w := 65000; m.l := -2000000000;
  m.flag := True; m.ch := 'Z'; m.c := Blue; m.sm := 9;
  copy := m;
  m.b := 0; m.s := 0; m.i := 0; m.w := 0; m.l := 0;
  WriteLn(copy.b, ' ', copy.s, ' ', copy.i, ' ', copy.w, ' ', copy.l);
  WriteLn(copy.flag, ' ', copy.ch, ' ', Ord(copy.c), ' ', copy.sm);
  WriteLn(m.b, m.s, m.i, m.w, m.l, ' ', m.ch = 'Z', ' ', m.c = Blue);
  arr[2] := copy; arr[3] := arr[2]; arr[1] := arr[3];
  arr[1].i := arr[1].i + 1;
  WriteLn(arr[1].i, ' ', arr[2].i, ' ', arr[3].i, ' ', arr[3].l);
  copy.b := copy.b + 10;
  WriteLn(copy.b, ' ', copy.w + copy.b)
end.
