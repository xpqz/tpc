program ide_files_open_prompt_ignores_extended_keys_and_control_codes;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('GOOD.TXT');
end.
