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

let get_item_priority ch =
  let code = Char.code ch in
  if code < 91 then (code mod 32) + 26 else code mod 32

let rec repeated_char_p1 str x =
  let half_size = String.length str / 2 in
  if x < half_size - 1 then
    let ch1 = String.get str x in
    if String.contains_from str half_size ch1 then get_item_priority ch1
    else repeated_char_p1 str (x + 1)
  else
    let ch2 = String.get str x in
    let str2 = String.sub str half_size half_size in
    if String.contains str2 ch2 then get_item_priority ch2
    else repeated_char_p1 str (x + 1)

let rec sum_priorities acc list =
  match list with
  | [] -> print_endline (string_of_int acc)
  | hd :: tl -> sum_priorities (acc + repeated_char_p1 hd 0) tl
;;

print_string "Total_p1: ";
sum_priorities 0 lines

let rec repeated_char_p2 (str1, str2, str3) x =
  let ch1 = String.get str1 x in
  if String.contains str2 ch1 && String.contains str3 ch1 then
    get_item_priority ch1
  else repeated_char_p2 (str1, str2, str3) (x + 1)

let rec traverse_lines acc lines =
  match lines with
  | h1 :: h2 :: h3 :: t ->
      let trio_priority = repeated_char_p2 (h1, h2, h3) 0 in
      traverse_lines (acc + trio_priority) t
  | _ -> print_endline (string_of_int acc)
;;

print_string "Total_p2: ";
traverse_lines 0 lines
