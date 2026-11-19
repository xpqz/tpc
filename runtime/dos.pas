{ The Dos unit: the clock and file-name routines of Turbo Pascal's Dos unit.

  Left out, because they need a directory listing, a disk or a changeable current directory
  that the flat virtual file system (docs/abi.md section 8) does not have: FindFirst, FindNext,
  FSearch, DiskSize, DiskFree, ChDir, GetFAttr, GetFTime. SearchRec is declared so that
  programs can name it; DosError is declared and never set.

  File names are plain string work. The current directory of every drive is its root, so
  FExpand puts the root of the drive in front of a relative name (the real oracle runs with
  C:\ as its current directory too).

  The host keeps no calendar for us. It hands over the local time as three numbers and the
  calendar is worked out here, in Pascal (docs/abi.md section 8, import `time`):
    time(0) takes a reading of the clock and returns the days since 1970-01-01,
    time(1) returns the seconds since midnight of that reading,
    time(2) returns its hundredths of a second.
  Both routines take their reading with time(0) first, so a date and a time never come from
  two different instants. 1970-01-01 was a Thursday. Dates before 1970 are not supported. }
unit Dos;

interface

const
  ReadOnly = $01;
  Hidden = $02;
  SysFile = $04;
  VolumeID = $08;
  Directory = $10;
  Archive = $20;
  AnyFile = $3F;

  fmClosed = $D7B0;
  fmInput = $D7B1;
  fmOutput = $D7B2;
  fmInOut = $D7B3;

type
  PathStr = string[79];
  DirStr = string[67];
  NameStr = string[8];
  ExtStr = string[4];

  DateTime = record
    Year, Month, Day, Hour, Min, Sec: Word;
  end;

  SearchRec = record
    Fill: array[1..21] of Byte;
    Attr: Byte;
    Time: LongInt;
    Size: LongInt;
    Name: string[12];
  end;

var
  DosError: Integer;

procedure GetDate(var Year, Month, Day, DayOfWeek: Word);
procedure GetTime(var Hour, Minute, Second, Sec100: Word);
procedure PackTime(var T: DateTime; var Time: LongInt);
procedure UnpackTime(Time: LongInt; var T: DateTime);
procedure FSplit(Path: PathStr; var Dir: DirStr; var Name: NameStr; var Ext: ExtStr);
function FExpand(Path: PathStr): PathStr;
procedure GetDir(Drive: Byte; var Path: string);

implementation

function HostTime(what: LongInt): LongInt; external 'time';

function IsLeapYear(year: LongInt): Boolean;
begin
  IsLeapYear := ((year mod 4 = 0) and (year mod 100 <> 0)) or (year mod 400 = 0)
end;

function DaysInYear(year: LongInt): LongInt;
begin
  if IsLeapYear(year) then
    DaysInYear := 366
  else
    DaysInYear := 365
end;

function DaysInMonth(year, month: LongInt): LongInt;
begin
  case month of
    4, 6, 9, 11: DaysInMonth := 30;
    2:
      if IsLeapYear(year) then
        DaysInMonth := 29
      else
        DaysInMonth := 28;
  else
    DaysInMonth := 31
  end
end;

procedure GetDate(var Year, Month, Day, DayOfWeek: Word);
var
  days, y, m: LongInt;
begin
  days := HostTime(0);
  { 1970-01-01 was a Thursday, 4 when Sunday is 0 as in Turbo Pascal. }
  DayOfWeek := (days + 4) mod 7;
  { Peel whole years, then whole months, off the day count. }
  y := 1970;
  while days >= DaysInYear(y) do
  begin
    days := days - DaysInYear(y);
    y := y + 1
  end;
  m := 1;
  while days >= DaysInMonth(y, m) do
  begin
    days := days - DaysInMonth(y, m);
    m := m + 1
  end;
  Year := y;
  Month := m;
  Day := days + 1
end;

procedure GetTime(var Hour, Minute, Second, Sec100: Word);
var
  days, seconds: LongInt;
begin
  { Taking the reading is the point; this routine has no use for the day count. }
  days := HostTime(0);
  seconds := HostTime(1);
  Hour := seconds div 3600;
  Minute := seconds mod 3600 div 60;
  Second := seconds mod 60;
  Sec100 := HostTime(2)
end;

{ The DOS packed time: seven bits of year since 1980 in the top of a LongInt, then four of
  month, five of day, five of hour, six of minute and five of seconds divided by two.
  The fields are added, not checked, so an out-of-range field spills into its neighbour;
  the year is cut to seven bits. }
procedure PackTime(var T: DateTime; var Time: LongInt);
begin
  Time := ((T.Year - 1980) and 127) shl 25 + LongInt(T.Month) shl 21 + LongInt(T.Day) shl 16
    + LongInt(T.Hour) shl 11 + LongInt(T.Min) shl 5 + T.Sec div 2
end;

{ Every field is read by shifting and masking, so any 32 bits unpack to something. }
procedure UnpackTime(Time: LongInt; var T: DateTime);
begin
  T.Year := 1980 + (Time shr 25) and 127;
  T.Month := (Time shr 21) and 15;
  T.Day := (Time shr 16) and 31;
  T.Hour := (Time shr 11) and 31;
  T.Min := (Time shr 5) and 63;
  T.Sec := (Time and 31) * 2
end;

{ The directory part ends at the last backslash or colon, the extension starts at the first
  dot after that. Each part is cut to the size of its type. }
procedure FSplit(Path: PathStr; var Dir: DirStr; var Name: NameStr; var Ext: ExtStr);
var
  i, dirEnd, dot: Integer;
begin
  dirEnd := 0;
  for i := 1 to Length(Path) do
    if (Path[i] = '\') or (Path[i] = ':') then
      dirEnd := i;
  dot := dirEnd + 1;
  while (dot <= Length(Path)) and (Path[dot] <> '.') do
    dot := dot + 1;
  Dir := Copy(Path, 1, dirEnd);
  Name := Copy(Path, dirEnd + 1, dot - dirEnd - 1);
  Ext := Copy(Path, dot, Length(Path))
end;

{ Removes the last component of a path that is kept without its leading backslash. }
procedure DropLastComponent(var below: string);
var
  cut: Integer;
begin
  cut := Length(below);
  while (cut > 0) and (below[cut] <> '\') do
    cut := cut - 1;
  if cut = 0 then
    below := ''
  else
    below := Copy(below, 1, cut - 1)
end;

{ A drive letter (any one character before a colon) or the default drive C:, then the
  components below the root: "." is dropped, ".." removes the component before it (the root
  has none to remove) and an empty component, from a doubled or trailing backslash, is kept.
  A closing backslash is added so that every component, the last too, ends at a backslash. }
function FExpand(Path: PathStr): PathStr;
var
  drive: Char;
  rest: string;
  below: string;
  component: string;
  count, i: Integer;
begin
  drive := 'C';
  rest := Path;
  if (Length(Path) >= 2) and (Path[2] = ':') then
  begin
    drive := Path[1];
    rest := Copy(Path, 3, 255)
  end;
  if (Length(rest) > 0) and (rest[1] = '\') then
    rest := Copy(rest, 2, 255);
  rest := rest + '\';
  below := '';
  component := '';
  count := 0;
  for i := 1 to Length(rest) do
    if rest[i] <> '\' then
      component := component + UpCase(rest[i])
    else
    begin
      if component = '..' then
      begin
        if count > 0 then
        begin
          DropLastComponent(below);
          count := count - 1
        end
      end
      else if component <> '.' then
      begin
        if count > 0 then
          below := below + '\';
        below := below + component;
        count := count + 1
      end;
      component := ''
    end;
  FExpand := UpCase(drive) + ':\' + below
end;

{ Drive 0 is the default drive C:, 1 is A:. }
procedure GetDir(Drive: Byte; var Path: string);
begin
  if Drive = 0 then
    Drive := 3;
  Path := Chr(64 + Drive) + ':\'
end;

end.
