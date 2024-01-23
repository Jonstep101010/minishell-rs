// "'"'"'test'"'"'"
// "'"     '"'      test   '"'    "'"
// '"test"'

// echo "'" '"' test "'" '"'
// '"test'"

// export TESTONE="'"test"'" -> "'" test "'"
// -> TESTONE='test'

// "'$USER'" - only in doublequotes expansion (nested single work)
// '$USER' - does not expand

// single quoted runs from one to the next (anything in between is a characterx)
// no way to use single quotes to denote single quotes, e.g. '''
// echo ''hello'' -> hello

// in case of minishell, only expansion is to be considered (no escapes)
// echo ""hello"" -> hello
// echo "'hello'" -> 'hello' (sq just characters)
// echo '"hello"' -> "hello"
// echo "$PATH" -> value of path
// echo ""'$PATH'"" -> prints $PATH bc singles & ignored ""

// echo "'$PATH'" -> value of path inside singlequotes
// echo "$PATH" -> value of path
// echo "$'PATH'" -> prints $'PATH'

// find next matching, (if next, ignore entirely)
// otherwise treat what is inside as literal/expand


// // cases:
// /*
// echo "hello tehre 'hello inside single' "jesus""
// -> hello tehre 'hello inside single' jesus

// echo 'hello tehre 'hello inside single' "jesus"'
// -> 'hello tehre 'hello inside single' "jesus"'

// echo 'hello tehre 'hello inside single' "jesus""'
// -> quotes not closed (error)

// echo 'hello tehre 'hello inside single' jesus""'
// -> hello tehre hello inside single jesus""

// echo 'hello tehre  'hello inside single' jesus'''
// -> hello tehre hello inside single jesus

// echo 'hello tehre  'hello inside single' jesus'
// -> hello tehre hello inside single jesus

// echo 'hello tehre 'hello inside single' ""jesus""'
// -> hello tehre hello inside single ""jesus""

// should not remove repeating doublequotes inside singles


// */
