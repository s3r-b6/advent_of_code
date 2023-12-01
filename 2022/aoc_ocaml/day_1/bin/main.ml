let lines =
  let file = "./bin/input.txt" in
  let content = In_channel.with_open_bin file In_channel.input_all in
  String.split_on_char '\n' content

let rec group input result =
  match input with
  | [] -> result
  | "" :: rest -> group rest (0 :: result)
  | hd :: tl ->
      group tl
        (match result with
        | [] -> [ int_of_string hd ]
        | h :: t -> (h + int_of_string hd) :: t)

let rec max3 input (m1, m2, m3) =
  match input with
  | [] -> (m1, m2, m3)
  | hd :: tl ->
      max3 tl
        (match (m1, m2, m3) with
        | m1, m2, _ when hd > m1 -> (hd, m1, m2)
        | m1, m2, _ when hd > m2 -> (m1, hd, m2)
        | m1, m2, m3 when hd > m3 -> (m1, m2, hd)
        | _ -> (m1, m2, m3))

let inventories = group lines [];;

print_string "Max: ";;
print_endline (string_of_int (List.fold_left max min_int inventories));;
print_string "Max3: ";;

let m1, m2, m3 = max3 inventories (0, 0, 0) in
print_endline (string_of_int (m1 + m2 + m3))
