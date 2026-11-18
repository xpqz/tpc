program CaseLongIntSelectorIsIntegerSized;
var l: LongInt;
begin
  l := 5;
  case l of
    -32768..-1: WriteLn('negative');
    0..32767: WriteLn('integer range')
  else
    WriteLn('other')
  end;
  l := 100000;
  case l of
    -32768..-1: WriteLn('negative');
    0..32767: WriteLn('integer range')
  else
    WriteLn('other')
  end
end.
