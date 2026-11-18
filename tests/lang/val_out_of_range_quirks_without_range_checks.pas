program ValQuirk;
var i, e: Integer; l: LongInt; b: Byte; w: Word;
begin
  Val('32768', i, e);
  WriteLn(i, ' ', e);
  Val('99999', i, e);
  WriteLn(i, ' ', e);
  Val('2147483647', l, e);
  WriteLn(l, ' ', e);
  Val('2147483648', l, e);
  WriteLn(l, ' ', e);
  Val('-2147483648', l, e);
  WriteLn(l, ' ', e);
  Val('255', b, e);
  WriteLn(b, ' ', e);
  Val('256', b, e);
  WriteLn(b, ' ', e);
  Val('65535', w, e);
  WriteLn(w, ' ', e);
  Val('65536', w, e);
  WriteLn(w, ' ', e);
  Val('-1', w, e);
  WriteLn(w, ' ', e)
end.
