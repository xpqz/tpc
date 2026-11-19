{ Menu: the menu bar on row 1 and its pull-down menus.

  Main paints with View.Paint, which writes a placeholder on row 1, and then calls DrawBar to
  write the real bar over it: the titles at the left and the current file name at the right
  end. Show draws the same bar, so the file name stays while a pull-down is open.
  Run opens the pull-down for the letter in Keys.MenuLetter (#0, from F10, opens the first
  menu) and returns the command chosen, in the codes of Keys. Run reads keys itself until a
  command is chosen or the menu is cancelled.

  The menus are one table of titles and one table of items; a menu owns the items from
  FirstItem to LastItem. A pull-down covers part of the edit area, so closing it, and
  switching to another menu, repaint the whole screen through View (decision 47) instead of
  saving what was underneath. There is no Edit menu: the plan lists no Edit command, so Alt-E
  opens nothing. Options has one placeholder item that only shows a message. }
unit Menu;

interface

procedure DrawBar;
procedure Show(Menu, Selected: Integer);
function Run(Letter: Char): Integer;

implementation

uses Crt, View, Keys, Files;

const
  MenuRow = 1;
  ScreenWidth = 80;
  PullDownRow = 2;
  PullDownWidth = 20;
  SelectedAttr = $07;   { light grey on black }
  EnterKey = #13;
  EscKey = #27;
  UpKey = #72;
  LeftKey = #75;
  RightKey = #77;
  DownKey = #80;
  F10Key = #68;

type
  Title = record
    Text: string[8];
    Letter: Char;               { the Alt key that opens the menu }
    FirstItem, LastItem: Integer;
  end;
  Item = record
    Text: string[PullDownWidth];  { the first letter after the blank picks the item }
    Command: Integer;
  end;

const
  MenuCount = 4;
  Titles: array[1..MenuCount] of Title = (
    (Text: 'File'; Letter: 'F'; FirstItem: 1; LastItem: 4),
    (Text: 'Run'; Letter: 'R'; FirstItem: 5; LastItem: 5),
    (Text: 'Compile'; Letter: 'C'; FirstItem: 6; LastItem: 6),
    (Text: 'Options'; Letter: 'O'; FirstItem: 7; LastItem: 7));
  Items: array[1..7] of Item = (
    (Text: ' New'; Command: CmdNew),
    (Text: ' Open          F3'; Command: CmdOpen),
    (Text: ' Save          F2'; Command: CmdSave),
    (Text: ' Quit        Alt-X'; Command: CmdQuit),
    (Text: ' Run       Ctrl-F9'; Command: CmdRun),
    (Text: ' Compile       F9'; Command: CmdCompile),
    (Text: ' Environment...'; Command: CmdNone));
  OptionsItem = 7;

function Padded(Text: string; Width: Integer): string;
begin
  while Length(Text) < Width do
    Text := Text + ' ';
  Padded := Text;
end;

{ The screen column of a title: the bar starts with a blank and each title is followed by
  two. }
function TitleColumn(Menu: Integer): Integer;
var
  Before, Column: Integer;
begin
  Column := 2;
  for Before := 1 to Menu - 1 do
    Inc(Column, Length(Titles[Before].Text) + 2);
  TitleColumn := Column;
end;

{ Writes the current file name at the right end of the bar. }
procedure WriteFileName;
var
  Name: string;
begin
  TextAttr := BarAttr;
  Name := ' ' + CurrentName + ' ';
  GotoXY(ScreenWidth - Length(Name) + 1, MenuRow);
  Write(Name);
end;

{ Writes the bar; the title of Open is shown selected, 0 selects none. The cursor stays where
  it was. }
procedure WriteBar(Open: Integer);
var
  Menu, Column: Integer;
  SavedX, SavedY: Byte;
begin
  SavedX := WhereX;
  SavedY := WhereY;
  TextAttr := BarAttr;
  GotoXY(1, MenuRow);
  Write(Padded('', 80));
  for Menu := 1 to MenuCount do
  begin
    if Menu = Open then
      TextAttr := SelectedAttr
    else
      TextAttr := BarAttr;
    GotoXY(TitleColumn(Menu) - 1, MenuRow);
    Write(' ' + Titles[Menu].Text + ' ');
  end;
  WriteFileName;
  GotoXY(SavedX, SavedY);
end;

procedure DrawBar;
begin
  WriteBar(0);
end;

{ The menu opened by an Alt letter, 0 when there is none. F10 (#0) opens the first menu. }
function MenuOfLetter(Letter: Char): Integer;
var
  Menu, Found: Integer;
begin
  Found := 0;
  if Letter = #0 then
    Found := 1;
  for Menu := 1 to MenuCount do
    if Titles[Menu].Letter = Letter then
      Found := Menu;
  MenuOfLetter := Found;
end;

{ Repaints the screen, then draws the bar with Menu open and its pull-down with the item
  Selected highlighted (an index into the item table). The cursor is left on the selected item.
  Run calls it after every key; it is public so that a test can look at an open menu without
  a key to wait for. }
procedure Show(Menu, Selected: Integer);
var
  Index: Integer;
  Column: Integer;
begin
  View.Paint;
  WriteBar(Menu);
  Column := TitleColumn(Menu) - 1;
  for Index := Titles[Menu].FirstItem to Titles[Menu].LastItem do
  begin
    if Index = Selected then
      TextAttr := SelectedAttr
    else
      TextAttr := BarAttr;
    GotoXY(Column, PullDownRow + Index - Titles[Menu].FirstItem);
    Write(Padded(Items[Index].Text, PullDownWidth));
  end;
  GotoXY(Column + 1, PullDownRow + Selected - Titles[Menu].FirstItem);
end;

{ The item of Menu whose letter is Key, or 0. }
function ItemOfLetter(Menu: Integer; Key: Char): Integer;
var
  Index, Found: Integer;
begin
  Found := 0;
  for Index := Titles[Menu].FirstItem to Titles[Menu].LastItem do
    if UpCase(Items[Index].Text[2]) = UpCase(Key) then
      Found := Index;
  ItemOfLetter := Found;
end;

{ Moves to the next or previous menu, wrapping round, and selects its first item. }
procedure MoveMenu(var Menu, Selected: Integer; Step: Integer);
begin
  Menu := Menu + Step;
  if Menu > MenuCount then
    Menu := 1;
  if Menu < 1 then
    Menu := MenuCount;
  Selected := Titles[Menu].FirstItem;
end;

{ Moves the selection by Step items, wrapping round inside the menu. }
procedure MoveItem(Menu: Integer; var Selected: Integer; Step: Integer);
begin
  Selected := Selected + Step;
  if Selected > Titles[Menu].LastItem then
    Selected := Titles[Menu].FirstItem;
  if Selected < Titles[Menu].FirstItem then
    Selected := Titles[Menu].LastItem;
end;

function Run(Letter: Char): Integer;
var
  Menu, Selected, Chosen: Integer;
  Key: Char;
  Finished: Boolean;
begin
  Run := CmdNone;
  Menu := MenuOfLetter(Letter);
  if Menu > 0 then
  begin
    Selected := Titles[Menu].FirstItem;
    Chosen := 0;
    Finished := False;
    while not Finished do
    begin
      Show(Menu, Selected);
      Key := ReadKey;
      if Key = #0 then
      begin
        Key := ReadKey;
        case Key of
          UpKey: MoveItem(Menu, Selected, -1);
          DownKey: MoveItem(Menu, Selected, 1);
          LeftKey: MoveMenu(Menu, Selected, -1);
          RightKey: MoveMenu(Menu, Selected, 1);
          F10Key: Finished := True;
        end;
      end
      else if Key = EnterKey then
      begin
        Chosen := Selected;
        Finished := True;
      end
      else if Key = EscKey then
        Finished := True
      else if ItemOfLetter(Menu, Key) > 0 then
      begin
        Chosen := ItemOfLetter(Menu, Key);
        Finished := True;
      end;
    end;
    if Chosen = OptionsItem then
      View.SetMessage('No options yet');
    View.Paint;
    DrawBar;
    if Chosen > 0 then
      Run := Items[Chosen].Command;
  end;
end;

end.
