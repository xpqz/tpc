program ide_files_open_modified_question_ignores_other_keys;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('mine');
  EditLoop;
  Report('GOOD.TXT');
end.
