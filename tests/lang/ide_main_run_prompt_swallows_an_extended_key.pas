program ide_main_run_prompt_swallows_an_extended_key;
uses Crt, Buffer, View, Keys, Files, FHarness, Main;

begin
  Start('a~b~c');
  RunIde;

end.
