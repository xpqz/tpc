program swag_dos_types;
uses Dos;
var
  sr: SearchRec;
  dt: DateTime;
  p: PathStr;
begin
  p := 'x';
  dt.Year := 1990;
  writeln(p, dt.Year);
end.
