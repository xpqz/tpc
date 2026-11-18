program NestedSameNameShadowing;
{ nested scopes reuse names: each reference resolves to the innermost declaration }
var v: Integer;
procedure A;
var v: Integer;
  procedure B;
  var v: Integer;
    procedure C;
    begin
      WriteLn('C sees ', v);
      v := v + 1
    end;
  begin
    v := 300;
    C;
    WriteLn('B ', v)
  end;
  procedure D;
  begin
    WriteLn('D sees ', v);
    v := v + 1
  end;
begin
  v := 200;
  B;
  D;
  WriteLn('A ', v)
end;
begin
  v := 100;
  A;
  WriteLn('main ', v)
end.
