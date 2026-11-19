program ide_keys_cmdnew_is_a_keys_constant;
uses Crt, Keys;

begin
  WriteLn(CmdNew, ' ', CmdQuit, ' ', CmdNew <> CmdNone);
end.
