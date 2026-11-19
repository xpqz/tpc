program ide_files_save_failure_keeps_the_text_modified;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('');
  EditLoop;
  Report('');
end.
