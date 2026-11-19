program ide_files_open_modified_escape_in_the_question_cancels_the_open;
uses Crt, Buffer, View, Keys, Files, FHarness;

begin
  Start('mine');
  EditLoop;
  Report('GOOD.TXT');
end.
