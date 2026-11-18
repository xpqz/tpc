program NestedLinkThroughUncle;
{ a routine calls a sibling of its parent: the callee must get the static link of the shared
  grandparent, not of the caller's own parent }
var trace: LongInt;
procedure Top(seed: Integer);
var shared: Integer;
  procedure Uncle(tag: Integer);
  begin
    shared := shared + tag;
    trace := trace * 10 + shared mod 10
  end;
  procedure Parent;
  var mine: Integer;
    procedure Child;
    begin
      mine := mine + 1;
      Uncle(mine);
      shared := shared + seed
    end;
  begin
    mine := 0;
    Child; Child;
    Uncle(100)
  end;
begin
  shared := seed;
  Parent;
  Uncle(0);
  WriteLn(shared)
end;
begin
  trace := 0;
  Top(2);
  WriteLn(trace);
  Top(7);
  WriteLn(trace)
end.
