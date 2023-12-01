let lines =
  let file = "./bin/input.txt" in
  let ic = In_channel.open_bin file in
  let rec read_lines acc =
    match In_channel.input_line ic with
    | Some line -> read_lines (line :: acc)
    | None -> acc
  in
  let result = read_lines [] in
  In_channel.close ic;
  result

let split_ranges str =
  match String.split_on_char ',' str with
  | [ r1; r2 ] -> (r1, r2)
  | _ -> assert false

let parse_range str =
  match String.split_on_char '-' str with
  | [ st; en ] -> (int_of_string st, int_of_string en)
  | _ -> assert false

let rec get_result_p1 acc lines =
  match lines with
  | [] -> print_endline (string_of_int acc)
  | hd :: tl ->
      let r1, r2 = split_ranges hd in
      let (st1, en1), (st2, en2) = (parse_range r1, parse_range r2) in
      if (st1 <= st2 && en1 >= en2) || (st2 <= st1 && en2 >= en1) then
        get_result_p1 (acc + 1) tl
      else get_result_p1 acc tl

let rec get_result_p2 acc lines =
  match lines with
  | [] -> print_endline (string_of_int acc)
  | hd :: tl ->
      let r1, r2 = split_ranges hd in
      let (st1, en1), (st2, en2) = (parse_range r1, parse_range r2) in
      if st2 > en1 || st1 > en2 then get_result_p2 acc tl
      else get_result_p2 (acc + 1) tl
;;

print_string "Part1: ";
get_result_p1 0 lines;

print_string "Part2: ";
get_result_p2 0 lines
