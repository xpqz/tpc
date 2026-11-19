unit Base;
interface
function Greeting: string;
implementation
function Greeting: string;
begin
  Greeting := 'base unit from a shared directory';
end;
end.
