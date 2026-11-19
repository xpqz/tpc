program TypedconstArrayIndexedByEnum;
type
  TDay = (Mon, Tue, Wed);
const
  names: array[TDay] of string[9] = ('Monday', 'Tuesday', 'Wednesday');
  hours: array[Mon..Wed] of Integer = (8, 6, 7);
var
  d: TDay;
begin
  for d := Mon to Wed do
    WriteLn(names[d], ' ', hours[d]);
end.
