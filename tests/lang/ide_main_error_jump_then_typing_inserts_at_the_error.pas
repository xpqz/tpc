program ide_main_error_jump_then_typing_inserts_at_the_error;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('begin~  x := ERROR + 1;~end.');
  RunIde;

end.
