{ The Dos unit: the clock routines of Turbo Pascal's Dos unit, GetDate and GetTime.

  The host keeps no calendar for us. It hands over the local time as three numbers and the
  calendar is worked out here, in Pascal (docs/abi.md section 8, import `time`):
    time(0) takes a reading of the clock and returns the days since 1970-01-01,
    time(1) returns the seconds since midnight of that reading,
    time(2) returns its hundredths of a second.
  Both routines take their reading with time(0) first, so a date and a time never come from
  two different instants. 1970-01-01 was a Thursday. Dates before 1970 are not supported. }
unit Dos;

interface

procedure GetDate(var Year, Month, Day, DayOfWeek: Word);
procedure GetTime(var Hour, Minute, Second, Sec100: Word);

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

end.
