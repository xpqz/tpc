program ide_files_save_empty_text_writes_an_empty_file;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('EMPTY2.PAS');
end.
