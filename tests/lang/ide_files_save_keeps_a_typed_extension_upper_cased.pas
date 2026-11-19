program ide_files_save_keeps_a_typed_extension_upper_cased;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('text');
  EditLoop;
  Report('NOTES.TXT');
end.
