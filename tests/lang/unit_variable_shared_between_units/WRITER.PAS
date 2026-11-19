unit Writer;
interface
procedure Put(n: Integer);
implementation
uses Store;
procedure Put(n: Integer);
begin
  Cell := n;
end;
end.
