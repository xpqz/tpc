program ide_files_status_question_for_a_named_text_on_open;
uses Crt, Buffer, View, Keys, Files, FHarness;

{ no Quit key: the program stops in its last ReadKey (exit 255) with the screen as that key
  left it, so the dump shows the status line (prompt, question or message) }
begin
  Start('');
  EditLoop;
end.
