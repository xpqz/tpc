program RecordsInOrderAndPartial;
type
  TPair = record
    first: Integer;
    second: Char;
    third: string[3];
    fourth: Boolean;
  end;
  TVariant = record
    case Boolean of
      True: (whole: Integer);
      False: (low, high: Byte);
  end;
const
  full: TPair = (first: -7; second: 'q'; third: 'abc'; fourth: True);
  prefix: TPair = (first: 1; second: 'k');
  stops: TPair = (first: 9);
  v: TVariant = (whole: 258);
begin
  WriteLn(full.first, ' ', full.second, ' ', full.third, ' ', full.fourth);
  WriteLn(prefix.first, ' ', prefix.second, ' ', Length(prefix.third), ' ', prefix.fourth);
  WriteLn(stops.first, ' ', Ord(stops.second), ' ', Length(stops.third), ' ', stops.fourth);
  WriteLn(v.whole, ' ', v.low, ' ', v.high);
  with full do
  begin
    first := first * 2;
    third := third + 'd';
  end;
  WriteLn(full.first, ' ', full.third);
end.
