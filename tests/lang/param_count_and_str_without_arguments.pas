program Params;
var i: Integer;
begin
  WriteLn(ParamCount);
  WriteLn(ParamCount = 0);
  WriteLn(ParamStr(1) = '', ' ', Length(ParamStr(1)));
  WriteLn(ParamStr(2) = '', ' ', ParamStr(100) = '');
  for i := 1 to ParamCount do WriteLn(ParamStr(i));
  WriteLn('done')
end.
