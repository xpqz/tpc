program TypedconstArrayOfRecords;
type
  TItem = record
    name: string[6];
    price: Integer;
  end;
const
  items: array[1..3] of TItem = (
    (name: 'pen'; price: 3),
    (name: 'book'; price: 12),
    (name: 'lamp'; price: 30));
var
  i, total: Integer;
begin
  total := 0;
  for i := 1 to 3 do
  begin
    WriteLn(items[i].name, ' ', items[i].price);
    total := total + items[i].price;
  end;
  WriteLn(total);
  items[2].price := items[2].price * 2;
  WriteLn(items[2].price);
end.
