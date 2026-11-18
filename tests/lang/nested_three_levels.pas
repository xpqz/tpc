program NestedThreeLevels;
{ three nesting levels, each reading the variables of the levels above }
procedure A;
var a: Integer;
  procedure B;
  var b: Integer;
    procedure C;
    var c: Integer;
    begin
      c := 3;
      WriteLn(a + b + c);
      a := 100
    end;
  begin
    b := 20;
    C;
    WriteLn(a + b)
  end;
begin
  a := 1;
  B;
  WriteLn(a)
end;
begin
  A
end.
