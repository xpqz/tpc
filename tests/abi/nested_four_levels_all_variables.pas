program NestedFourLevelsAllVariables;
{ the innermost routine reads and writes locals of all three enclosing routines }
procedure L1;
var a: Integer;
  procedure L2;
  var b: Integer;
    procedure L3;
    var c: Integer;
      procedure L4;
      var d: Integer;
      begin
        d := 4;
        a := a + 1; b := b + 2; c := c + 3;
        WriteLn(a, ' ', b, ' ', c, ' ', d)
      end;
    begin
      c := 30;
      L4;
      L4;
      WriteLn('c ', c)
    end;
  begin
    b := 20;
    L3;
    WriteLn('b ', b)
  end;
begin
  a := 10;
  L2;
  WriteLn('a ', a)
end;
begin
  L1
end.
