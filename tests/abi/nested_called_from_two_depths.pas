program NestedCalledFromTwoDepths;
{ one nested routine called from the parent and from a deeper sibling's child }
procedure P;
var n: Integer;
  procedure Inc1;
  begin
    n := n + 1
  end;
  procedure Q;
    procedure R;
    begin
      Inc1;
      Inc1
    end;
  begin
    R;
    Inc1
  end;
begin
  n := 0;
  Inc1;
  Q;
  WriteLn(n);
  Q;
  WriteLn(n)
end;
begin
  P
end.
