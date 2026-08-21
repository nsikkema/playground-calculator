use shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Add translations for the Expression Engine to the provided `SharedStringTranslationMap`.
pub(crate) fn add_expression_engine_translation_map(
    translation_map: &mut SharedStringTranslationMap,
) {
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_character",
        HashMap::from([
            ("en", "Invalid character in expression: '%{character}'"),
            ("zh", "表达式中存在无效字符：'%{character}'"),
            ("de", "Ungültiges Zeichen im Ausdruck: '%{character}'"),
            ("es", "Carácter no válido en la expresión: '%{character}'"),
            (
                "fr",
                "Caractère non valide dans l’expression : '%{character}'",
            ),
            ("ja", "式に無効な文字があります: '%{character}'"),
            ("ko", "식에 잘못된 문자가 있습니다: '%{character}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_number",
        HashMap::from([
            ("en", "Invalid number in expression: '%{number}'"),
            ("zh", "表达式中的数字无效：'%{number}'"),
            ("de", "Ungültige Zahl im Ausdruck: '%{number}'"),
            ("es", "Número no válido en la expresión: '%{number}'"),
            ("fr", "Nombre non valide dans l’expression : '%{number}'"),
            ("ja", "式に無効な数値があります: '%{number}'"),
            ("ko", "식에 잘못된 숫자가 있습니다: '%{number}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_operator",
        HashMap::from([
            ("en", "Invalid operator in expression: '%{operator}'"),
            ("zh", "表达式中的运算符无效：'%{operator}'"),
            ("de", "Ungültiger Operator im Ausdruck: '%{operator}'"),
            ("es", "Operador no válido en la expresión: '%{operator}'"),
            (
                "fr",
                "Opérateur non valide dans l’expression : '%{operator}'",
            ),
            ("ja", "式に無効な演算子があります: '%{operator}'"),
            ("ko", "식에 잘못된 연산자가 있습니다: '%{operator}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_invalid_string",
        HashMap::from([
            ("en", "Invalid string in expression: '%{string}'"),
            ("zh", "表达式中的字符串无效：'%{string}'"),
            ("de", "Ungültige Zeichenkette im Ausdruck: '%{string}'"),
            ("es", "Cadena no válida en la expresión: '%{string}'"),
            ("fr", "Chaîne non valide dans l’expression : '%{string}'"),
            ("ja", "式に無効な文字列があります: '%{string}'"),
            ("ko", "식에 잘못된 문자열이 있습니다: '%{string}'"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_lexer_unterminated_string_literal",
        HashMap::from([
            ("en", "Unterminated string literal"),
            ("zh", "未终止的字符串字面量"),
            ("de", "Nicht abgeschlossene Zeichenkette"),
            ("es", "Literal de cadena sin terminar"),
            ("fr", "Littéral de chaîne non terminé"),
            ("ja", "文字列リテラルが終了していません"),
            ("ko", "문자열 리터럴이 종료되지 않았습니다"),
        ]),
    );

    translation_map.set_translation_key(
        "expression_engine_evaluation_function_name_empty",
        HashMap::from([
            ("en", "Function name must not be empty."),
            ("zh", "函数名称不能为空。"),
            ("de", "Der Funktionsname darf nicht leer sein."),
            ("es", "El nombre de la función no debe estar vacío."),
            ("fr", "Le nom de la fonction ne doit pas être vide."),
            ("ja", "関数名を空にすることはできません。"),
            ("ko", "함수 이름은 비워둘 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_global",
        HashMap::from([
            ("en", "Missing required global: %{global}"),
            ("zh", "缺少必需的全局：%{global}"),
            ("de", "Fehlendes erforderliches globales Element: %{global}"),
            ("es", "Falta global requerido: %{global}"),
            ("fr", "Global requis manquant : %{global}"),
            ("ja", "必要なグローバルが欠落しています: %{global}"),
            ("ko", "필수 전역 누락: %{global}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_parameter",
        HashMap::from([
            ("en", "Missing required parameter: %{parameter}"),
            ("zh", "缺少必需参数：%{parameter}"),
            ("de", "Fehlender erforderlicher Parameter: %{parameter}"),
            ("es", "Falta el parámetro requerido: %{parameter}"),
            ("fr", "Paramètre obligatoire manquant : %{parameter}"),
            ("ja", "必須パラメータが欠落しています: %{parameter}"),
            ("ko", "필수 매개변수 누락: %{parameter}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_variable",
        HashMap::from([
            ("en", "Missing required variable: %{variable}"),
            ("zh", "缺少必需变量：%{variable}"),
            ("de", "Fehlende erforderliche Variable: %{variable}"),
            ("es", "Falta la variable requerida: %{variable}"),
            ("fr", "Variable obligatoire manquante : %{variable}"),
            ("ja", "必要な変数がありません: %{variable}"),
            ("ko", "필수 변수 누락: %{variable}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_required_function",
        HashMap::from([
            ("en", "Missing required function: %{function}"),
            ("zh", "缺少所需功能：%{function}"),
            ("de", "Fehlende erforderliche Funktion: %{function}"),
            ("es", "Falta la función requerida: %{function}"),
            ("fr", "Fonction requise manquante : %{function}"),
            ("ja", "必要な関数がありません: %{function}"),
            ("ko", "필수 기능 누락: %{function}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_end_of_input",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected end of input, found %{token}",
            ),
            ("zh", "表达式无效：预期输入结束，但发现 %{token}"),
            (
                "de",
                "Ungültiger Ausdruck: erwartetes Ende der Eingabe, gefunden %{token}",
            ),
            (
                "es",
                "Expresión no válida: final esperado de la entrada, encontrado %{token}",
            ),
            (
                "fr",
                "Expression non valide : fin attendue de l'entrée, trouvée %{token}",
            ),
            (
                "ja",
                "無効な式: 入力の終わりが予期されましたが、%{token} が見つかりました",
            ),
            ("ko", "잘못된 표현식: 예상된 입력 끝, %{token}가 발견됨"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_expression",
        HashMap::from([
            ("en", "Invalid expression: expected an identifier, a number, or a prefix operator, found %{token}"),
            ("zh", "表达式无效：需要标识符、数字或前缀运算符，但找到 %{token}"),
            ("de", "Ungültiger Ausdruck: Erwartet wurde ein Bezeichner, eine Zahl oder ein Präfixoperator. %{token} wurde gefunden"),
            ("es", "Expresión no válida: se esperaba un identificador, un número o un operador de prefijo, se encontró %{token}"),
            ("fr", "Expression non valide : attendu un identifiant, un nombre ou un opérateur de préfixe, trouvé %{token}"),
            ("ja", "無効な式: 識別子、数値、または接頭辞演算子が必要でしたが、%{token} が見つかりました。"),
            ("ko", "잘못된 표현식: 식별자, 숫자 또는 접두사 연산자가 필요합니다. %{token}가 발견되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_operator",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected an operator, found %{token}",
            ),
            ("zh", "表达式无效：需要一个运算符，但找到了 %{token}"),
            (
                "de",
                "Ungültiger Ausdruck: Operator erwartet, %{token} gefunden",
            ),
            (
                "es",
                "Expresión no válida: se esperaba un operador, se encontró %{token}",
            ),
            (
                "fr",
                "Expression non valide : opérateur attendu, %{token} trouvé",
            ),
            (
                "ja",
                "無効な式: 演算子が必要でしたが、%{token} が見つかりました",
            ),
            (
                "ko",
                "잘못된 표현식: 연산자가 필요했지만 %{token}가 발견되었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_expected_specific_operator",
        HashMap::from([
            (
                "en",
                "Invalid expression: expected operator '%{expected}', found %{token}",
            ),
            (
                "zh",
                "表达式无效：预期的运算符“%{expected}”，但找到了 %{token}",
            ),
            (
                "de",
                "Ungültiger Ausdruck: erwarteter Operator „%{expected}“, gefunden %{token}",
            ),
            (
                "es",
                "Expresión no válida: operador esperado '%{expected}', encontrado %{token}",
            ),
            (
                "fr",
                "Expression non valide : opérateur attendu '%{expected}', trouvé %{token}",
            ),
            (
                "ja",
                "無効な式: 予期された演算子 '%{expected}'、%{token} が見つかりました",
            ),
            (
                "ko",
                "잘못된 표현식: 예상된 연산자 '%{expected}', 발견된 %{token}",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_number",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found number %{value}"),
            ("zh", "表达式无效：函数调用需要函数名称，找到编号 %{value}"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen, gefundene Nummer %{value}"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función, número encontrado %{value}"),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, numéro trouvé %{value}"),
            ("ja", "無効な式: 関数呼び出しには関数名が必要ですが、番号 %{value} が見つかりました。"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요합니다. 발견된 번호는 %{value}입니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_text",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found \"%{value}\""),
            ("zh", "表达式无效：函数调用需要函数名称，找到“%{value}”"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen, gefunden „%{value}“"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función, encontrada \"%{value}\""),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, trouvé \"%{value}\""),
            ("ja", "無効な式: 関数呼び出しには関数名が必要です。\"%{value}\" が見つかりました。"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요합니다. \"%{value}\"가 발견되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_function_name_required_expression",
        HashMap::from([
            ("en", "Invalid expression: function calls require a function name, found expression starting with operator %{operator}"),
            ("zh", "表达式无效：函数调用需要函数名称，找到以运算符 %{operator} 开头的表达式"),
            ("de", "Ungültiger Ausdruck: Funktionsaufrufe erfordern einen Funktionsnamen. Der gefundene Ausdruck beginnt mit dem Operator %{operator}"),
            ("es", "Expresión no válida: las llamadas a funciones requieren un nombre de función; se encontró una expresión que comienza con el operador %{operator}"),
            ("fr", "Expression non valide : les appels de fonction nécessitent un nom de fonction, expression trouvée commençant par l'opérateur %{operator}"),
            ("ja", "無効な式: 関数呼び出しには関数名が必要ですが、演算子 %{operator} で始まる式が見つかりました"),
            ("ko", "잘못된 표현식: 함수 호출에는 함수 이름이 필요합니다. 연산자 %{operator}로 시작하는 표현식이 발견되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_parser_invalid_prefix_operator",
        HashMap::from([
            ("en", "Invalid prefix operator in expression: '%{operator}'"),
            ("zh", "表达式中的前缀运算符无效：“%{operator}”"),
            ("de", "Ungültiger Präfixoperator im Ausdruck: „%{operator}“"),
            (
                "es",
                "Operador de prefijo no válido en la expresión: '%{operator}'",
            ),
            (
                "fr",
                "Opérateur de préfixe non valide dans l'expression : '%{operator}'",
            ),
            ("ja", "式内の接頭辞演算子が無効です: '%{operator}'"),
            (
                "ko",
                "표현식의 접두사 연산자가 잘못되었습니다: '%{operator}'",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_binary_missing_left_operand",
        HashMap::from([
            ("en", "Binary operator is missing its left operand."),
            ("zh", "二元运算符缺少左操作数。"),
            ("de", "Dem Binäroperator fehlt der linke Operand."),
            ("es", "Al operador binario le falta su operando izquierdo."),
            ("fr", "L'opérateur binaire n'a pas son opérande gauche."),
            ("ja", "二項演算子に左オペランドがありません。"),
            ("ko", "이항 연산자에 왼쪽 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_binary_missing_right_operand",
        HashMap::from([
            ("en", "Binary operator is missing its right operand."),
            ("zh", "二元运算符缺少右操作数。"),
            ("de", "Dem Binäroperator fehlt der richtige Operand."),
            ("es", "Al operador binario le falta su operando derecho."),
            ("fr", "L'opérateur binaire n'a pas son opérande droit."),
            ("ja", "二項演算子に右側のオペランドがありません。"),
            ("ko", "이진 연산자에 오른쪽 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unary_missing_operand",
        HashMap::from([
            ("en", "Unary operator is missing its operand."),
            ("zh", "一元运算符缺少操作数。"),
            ("de", "Dem unären Operator fehlt sein Operand."),
            ("es", "Al operador unario le falta su operando."),
            ("fr", "L'opérateur unaire n'a pas son opérande."),
            ("ja", "単項演算子にオペランドがありません。"),
            ("ko", "단항 연산자에 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unary_plus_missing_operand",
        HashMap::from([
            ("en", "Unary '+' operator is missing its operand."),
            ("zh", "一元“+”运算符缺少其操作数。"),
            ("de", "Dem unären „+“-Operator fehlt sein Operand."),
            ("es", "Al operador unario '+' le falta su operando."),
            ("fr", "L'opérateur unaire '+' n'a pas son opérande."),
            ("ja", "単項「+」演算子にオペランドがありません。"),
            ("ko", "단항 '+' 연산자에 피연산자가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_index_missing_target",
        HashMap::from([
            ("en", "Index operator is missing its target."),
            ("zh", "索引运算符缺少其目标。"),
            ("de", "Der Indexoperator verfehlt sein Ziel."),
            ("es", "El operador de índice no ha alcanzado su objetivo."),
            ("fr", "L'opérateur d'index manque sa cible."),
            ("ja", "インデックス演算子にターゲットがありません。"),
            ("ko", "인덱스 연산자에 대상이 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_index_missing_index",
        HashMap::from([
            ("en", "Index operator is missing its index."),
            ("zh", "索引运算符缺少索引。"),
            ("de", "Dem Indexoperator fehlt sein Index."),
            ("es", "Al operador de índice le falta su índice."),
            ("fr", "L’opérateur d’index n’a pas son index."),
            ("ja", "インデックス演算子にインデックスがありません。"),
            ("ko", "인덱스 연산자에 인덱스가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_invalid_numeric_literal",
        HashMap::from([
            ("en", "Invalid numeric literal: %{value}"),
            ("zh", "无效的数字文字：%{value}"),
            ("de", "Ungültiges numerisches Literal: %{value}"),
            ("es", "Literal numérico no válido: %{value}"),
            ("fr", "Littéral numérique non valide : %{value}"),
            ("ja", "無効な数値リテラル: %{value}"),
            ("ko", "잘못된 숫자 리터럴: %{value}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_translator_unsupported_operator",
        HashMap::from([
            ("en", "Unsupported operator: %{operator}"),
            ("zh", "不支持的运算符：%{operator}"),
            ("de", "Nicht unterstützter Operator: %{operator}"),
            ("es", "Operador no admitido: %{operator}"),
            ("fr", "Opérateur non pris en charge : %{operator}"),
            ("ja", "サポートされていない演算子: %{operator}"),
            ("ko", "지원되지 않는 연산자: %{operator}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_floating_point_not_finite",
        HashMap::from([
            ("en", "Floating-point results must be finite."),
            ("zh", "浮点结果必须是有限的。"),
            ("de", "Gleitkommaergebnisse müssen endlich sein."),
            ("es", "Los resultados de punto flotante deben ser finitos."),
            (
                "fr",
                "Les résultats à virgule flottante doivent être finis.",
            ),
            ("ja", "浮動小数点の結果は有限でなければなりません。"),
            ("ko", "부동 소수점 결과는 유한해야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_variable_not_found",
        HashMap::from([
            ("en", "Variable '%{variable}' not found in computed data. If you want to use a literal string, wrap it in quotes \"%{variable}\"."),
            ("zh", "在计算数据中找不到变量“%{variable}”。如果要使用文字字符串，请将其用引号 \"%{variable}\" 括起来。"),
            ("de", "Die Variable „%{variable}“ wurde in den berechneten Daten nicht gefunden. Wenn Sie eine Literalzeichenfolge verwenden möchten, schließen Sie sie in Anführungszeichen „%{variable}“ ein."),
            ("es", "La variable '%{variable}' no se encuentra en los datos calculados. Si desea utilizar una cadena literal, envuélvala entre comillas \"%{variable}\"."),
            ("fr", "La variable '%{variable}' est introuvable dans les données calculées. Si vous souhaitez utiliser une chaîne littérale, placez-la entre guillemets \"%{variable}\"."),
            ("ja", "変数「%{variable}」が計算データに見つかりません。リテラル文字列を使用する場合は、引用符 \"%{variable}\" で囲みます。"),
            ("ko", "계산된 데이터에서 변수 '%{variable}'를 찾을 수 없습니다. 리터럴 문자열을 사용하려면 따옴표 \"%{variable}\"로 묶습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_integer_overflow",
        HashMap::from([
            ("en", "Integer overflow."),
            ("zh", "整数溢出。"),
            ("de", "Ganzzahlüberlauf."),
            ("es", "Desbordamiento de enteros."),
            ("fr", "Débordement d'entier."),
            ("ja", "整数オーバーフロー。"),
            ("ko", "정수 오버플로."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unary_operation",
        HashMap::from([
            ("en", "Invalid unary operation."),
            ("zh", "无效的一元运算。"),
            ("de", "Ungültige unäre Operation."),
            ("es", "Operación unaria no válida."),
            ("fr", "Opération unaire invalide."),
            ("ja", "無効な単項演算です。"),
            ("ko", "단항 연산이 잘못되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unsupported_operator",
        HashMap::from([
            ("en", "Unsupported operator %{operator} for %{type} values."),
            ("zh", "对于 %{type} 值，不支持运算符 %{operator}。"),
            (
                "de",
                "Nicht unterstützter Operator %{operator} für %{type}-Werte.",
            ),
            (
                "es",
                "Operador no admitido %{operator} para valores %{type}.",
            ),
            (
                "fr",
                "Opérateur %{operator} non pris en charge pour les valeurs %{type}.",
            ),
            (
                "ja",
                "%{type} 値の演算子 %{operator} はサポートされていません。",
            ),
            (
                "ko",
                "%{type} 값에 대해 지원되지 않는 연산자 %{operator}입니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_division_by_zero",
        HashMap::from([
            ("en", "Division by zero."),
            ("zh", "除以零。"),
            ("de", "Division durch Null."),
            ("es", "División por cero."),
            ("fr", "Division par zéro."),
            ("ja", "ゼロ除算。"),
            ("ko", "0으로 나누기."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_modulus_by_zero",
        HashMap::from([
            ("en", "Modulus by zero."),
            ("zh", "模数为零。"),
            ("de", "Modul um Null."),
            ("es", "Módulo por cero."),
            ("fr", "Module par zéro."),
            ("ja", "ゼロによるモジュラス。"),
            ("ko", "모듈러스는 0입니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_integer_exponent",
        HashMap::from([
            (
                "en",
                "Integer exponent must be non-negative and fit within u32.",
            ),
            ("zh", "整数指数必须是非负数并且适合 u32。"),
            (
                "de",
                "Der ganzzahlige Exponent darf nicht negativ sein und in u32 passen.",
            ),
            (
                "es",
                "El exponente entero debe ser no negativo y caber dentro de u32.",
            ),
            (
                "fr",
                "L'exposant entier doit être non négatif et correspondre à u32.",
            ),
            (
                "ja",
                "整数の指数は負ではなく、u32 内に収まる必要があります。",
            ),
            (
                "ko",
                "정수 지수는 음수가 아니어야 하며 u32 내에 맞아야 합니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_not_defined",
        HashMap::from([
            ("en", "Function '%{function}' is not defined."),
            ("zh", "未定义函数“%{function}”。"),
            ("de", "Die Funktion „%{function}“ ist nicht definiert."),
            ("es", "La función '%{function}' no está definida."),
            ("fr", "La fonction '%{function}' n'est pas définie."),
            ("ja", "関数「%{function}」が定義されていません。"),
            ("ko", "'%{function}' 기능이 정의되지 않았습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_exact",
        HashMap::from([
            ("en", "Function '%{function}' requires exactly %{expected} arguments, got %{actual}."),
            ("zh", "函数“%{function}”恰好需要 %{expected} 参数，得到 %{actual}。"),
            ("de", "Die Funktion „%{function}“ erfordert genau die Argumente %{expected} und hat %{actual}."),
            ("es", "La función '%{function}' requiere exactamente los argumentos %{expected}, obtuvo %{actual}."),
            ("fr", "La fonction '%{function}' nécessite exactement les arguments %{expected}, j'ai obtenu %{actual}."),
            ("ja", "関数 '%{function}' には %{expected} 引数が必要ですが、%{actual} を取得しました。"),
            ("ko", "'%{function}' 함수에는 정확히 %{expected} 인수가 필요하며 %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_minimum",
        HashMap::from([
            ("en", "Function '%{function}' requires at least %{minimum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”至少需要 %{minimum} 参数，得到 %{actual}。"),
            ("de", "Die Funktion „%{function}“ erfordert mindestens %{minimum}-Argumente und hat %{actual}."),
            ("es", "La función '%{function}' requiere al menos argumentos %{minimum}, obtuvo %{actual}."),
            ("fr", "La fonction '%{function}' nécessite au moins des arguments %{minimum}, a obtenu %{actual}."),
            ("ja", "関数 '%{function}' には少なくとも %{minimum} 引数が必要です。%{actual} を取得しました。"),
            ("ko", "함수 '%{function}'에는 최소한 %{minimum} 인수가 필요하며 %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_maximum",
        HashMap::from([
            ("en", "Function '%{function}' allows at most %{maximum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”最多允许 %{maximum} 参数，得到 %{actual}。"),
            ("de", "Die Funktion „%{function}“ erlaubt höchstens %{maximum}-Argumente und hat %{actual}."),
            ("es", "La función '%{function}' permite como máximo argumentos %{maximum}, obtuvo %{actual}."),
            ("fr", "La fonction '%{function}' autorise au plus les arguments %{maximum}, obtenu %{actual}."),
            ("ja", "関数 '%{function}' は最大 %{maximum} 引数を許可します。%{actual} を取得しました。"),
            ("ko", "함수 '%{function}'는 최대 %{maximum} 인수를 허용하며 %{actual}를 얻습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_function_wrong_argument_count_range",
        HashMap::from([
            ("en", "Function '%{function}' requires between %{minimum} and %{maximum} arguments, got %{actual}."),
            ("zh", "函数“%{function}”需要 %{minimum} 和 %{maximum} 之间的参数，得到 %{actual}。"),
            ("de", "Die Funktion „%{function}“ erfordert Argumente zwischen %{minimum} und %{maximum} und hat %{actual}."),
            ("es", "La función '%{function}' requiere entre los argumentos %{minimum} y %{maximum}, obtuvo %{actual}."),
            ("fr", "La fonction '%{function}' nécessite entre les arguments %{minimum} et %{maximum}, a obtenu %{actual}."),
            ("ja", "関数 '%{function}' には %{minimum} と %{maximum} の間の引数が必要ですが、%{actual} を取得しました。"),
            ("ko", "'%{function}' 함수에는 %{minimum}와 %{maximum} 인수가 필요하며 %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_index_count",
        HashMap::from([
            ("en", "Indexing requires exactly 2 or 4 indices, got %{actual}."),
            ("zh", "索引正好需要 2 或 4 个索引，得到 %{actual}。"),
            ("de", "Die Indizierung erfordert genau 2 oder 4 Indizes, erhalten Sie %{actual}."),
            ("es", "La indexación requiere exactamente 2 o 4 índices, obtuve %{actual}."),
            ("fr", "L'indexation nécessite exactement 2 ou 4 indices, obtenu %{actual}."),
            ("ja", "インデックス作成には 2 つまたは 4 つのインデックスが必要です。%{actual} を取得しました。"),
            ("ko", "인덱싱에는 정확히 2개 또는 4개의 인덱스가 필요합니다. %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_first_index",
        HashMap::from([
            ("en", "Missing first index."),
            ("zh", "缺少第一个索引。"),
            ("de", "Fehlender erster Index."),
            ("es", "Falta el primer índice."),
            ("fr", "Premier index manquant."),
            ("ja", "最初のインデックスがありません。"),
            ("ko", "첫 번째 색인이 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_missing_second_index",
        HashMap::from([
            ("en", "Missing second index."),
            ("zh", "缺少第二个索引。"),
            ("de", "Fehlender zweiter Index."),
            ("es", "Falta el segundo índice."),
            ("fr", "Manque le deuxième index."),
            ("ja", "2 番目のインデックスがありません。"),
            ("ko", "두 번째 색인이 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_for_indexing",
        HashMap::from([
            ("en", "Expected a table for table indexing."),
            ("zh", "需要一个用于表索引的表。"),
            (
                "de",
                "Erwartet wurde eine Tabelle für die Tabellenindizierung.",
            ),
            ("es", "Se esperaba una tabla para indexar tablas."),
            ("fr", "Attendu une table pour l’indexation des tables."),
            ("ja", "テーブルのインデックス付けにはテーブルが必要です。"),
            ("ko", "테이블 인덱싱을 위한 테이블이 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_row_index_out_of_bounds",
        HashMap::from([
            ("en", "Row index %{index} is out of bounds for a table with %{count} rows."),
            ("zh", "对于包含 %{count} 行的表，行索引 %{index} 超出范围。"),
            ("de", "Der Zeilenindex %{index} liegt außerhalb des zulässigen Bereichs für eine Tabelle mit %{count}-Zeilen."),
            ("es", "El índice de filas %{index} está fuera de los límites de una tabla con filas %{count}."),
            ("fr", "L'index de ligne %{index} est hors limites pour une table comportant des lignes %{count}."),
            ("ja", "行インデックス %{index} は、%{count} 行を持つテーブルの範囲外です。"),
            ("ko", "행 인덱스 %{index}는 %{count} 행이 있는 테이블의 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_row_index",
        HashMap::from([
            (
                "en",
                "Expected an integer index for the table, got %{actual}.",
            ),
            ("zh", "需要表的整数索引，得到 %{actual}。"),
            (
                "de",
                "Erwartete einen ganzzahligen Index für die Tabelle, erhielt %{actual}.",
            ),
            (
                "es",
                "Se esperaba un índice entero para la tabla, obtuve %{actual}.",
            ),
            (
                "fr",
                "Attendu un index entier pour la table, j'ai obtenu %{actual}.",
            ),
            (
                "ja",
                "テーブルの整数インデックスが予期されましたが、%{actual} を取得しました。",
            ),
            (
                "ko",
                "테이블에 대한 정수 인덱스가 필요했는데 %{actual}를 얻었습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_field_not_found",
        HashMap::from([
            ("en", "Field '%{field}' not found in the table row."),
            ("zh", "在表行中找不到字段“%{field}”。"),
            (
                "de",
                "Das Feld „%{field}“ wurde in der Tabellenzeile nicht gefunden.",
            ),
            (
                "es",
                "El campo '%{field}' no se encuentra en la fila de la tabla.",
            ),
            (
                "fr",
                "Champ « %{field} » introuvable dans la ligne du tableau.",
            ),
            ("ja", "フィールド「%{field}」がテーブル行に見つかりません。"),
            ("ko", "테이블 행에서 '%{field}' 필드를 찾을 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_index_out_of_bounds",
        HashMap::from([
            ("en", "Column index %{index} is out of bounds for a table with %{count} columns."),
            ("zh", "对于具有 %{count} 列的表，列索引 %{index} 超出范围。"),
            ("de", "Der Spaltenindex %{index} liegt außerhalb des zulässigen Bereichs für eine Tabelle mit %{count}-Spalten."),
            ("es", "El índice de columna %{index} está fuera de los límites de una tabla con columnas %{count}."),
            ("fr", "L'index de colonne %{index} est hors limites pour une table comportant des colonnes %{count}."),
            ("ja", "列インデックス %{index} は、%{count} 列を持つテーブルの範囲外です。"),
            ("ko", "열 인덱스 %{index}는 %{count} 열이 있는 테이블의 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_field_index",
        HashMap::from([
            ("en", "Expected a string or integer index for the table field, got %{actual}."),
            ("zh", "需要表字段的字符串或整数索引，得到 %{actual}。"),
            ("de", "Für das Tabellenfeld wurde ein String- oder Integer-Index erwartet, %{actual} erhalten."),
            ("es", "Se esperaba una cadena o un índice entero para el campo de la tabla, obtuve %{actual}."),
            ("fr", "Attendu une chaîne ou un index entier pour le champ de la table, j'ai obtenu %{actual}."),
            ("ja", "テーブル フィールドに文字列または整数のインデックスが必要でしたが、%{actual} を取得しました。"),
            ("ko", "테이블 필드에 문자열 또는 정수 인덱스가 필요합니다. %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_choice",
        HashMap::from([
            ("en", "Value '%{value}' is not a valid choice."),
            ("zh", "值“%{value}”不是有效的选择。"),
            ("de", "Der Wert „%{value}“ ist keine gültige Auswahl."),
            ("es", "El valor '%{value}' no es una opción válida."),
            ("fr", "La valeur « %{value} » n'est pas un choix valide."),
            ("ja", "値「%{value}」は有効な選択ではありません。"),
            ("ko", "'%{value}' 값은 유효한 선택이 아닙니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unit_id",
        HashMap::from([
            ("en", "Value '%{value}' is not a valid unit ID."),
            ("zh", "值“%{value}”不是有效的单位 ID。"),
            ("de", "Der Wert „%{value}“ ist keine gültige Einheiten-ID."),
            ("es", "El valor '%{value}' no es un ID de unidad válido."),
            (
                "fr",
                "La valeur « %{value} » n'est pas un ID d'unité valide.",
            ),
            ("ja", "値「%{value}」は有効なユニット ID ではありません。"),
            ("ko", "값 '%{value}'는 유효한 장치 ID가 아닙니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_invalid_unit_for_family",
        HashMap::from([
            (
                "en",
                "Value '%{value}' is not a valid unit ID for %{family}.",
            ),
            ("zh", "值“%{value}”不是 %{family} 的有效单位 ID。"),
            (
                "de",
                "Der Wert „%{value}“ ist keine gültige Einheiten-ID für %{family}.",
            ),
            (
                "es",
                "El valor '%{value}' no es un ID de unidad válido para %{family}.",
            ),
            (
                "fr",
                "La valeur « %{value} » n'est pas un ID d'unité valide pour %{family}.",
            ),
            (
                "ja",
                "値「%{value}」は、%{family} の有効なユニット ID ではありません。",
            ),
            (
                "ko",
                "값 '%{value}'는 %{family}의 유효한 장치 ID가 아닙니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_unit_value",
        HashMap::from([
            ("en", "Expected a unit value, but got %{actual}."),
            ("zh", "预期有一个单位值，但得到的是 %{actual}。"),
            (
                "de",
                "Einen Einheitenwert erwartet, aber %{actual} erhalten.",
            ),
            (
                "es",
                "Se esperaba un valor unitario, pero obtuve %{actual}.",
            ),
            (
                "fr",
                "Je m'attendais à une valeur unitaire, mais j'ai obtenu %{actual}.",
            ),
            (
                "ja",
                "ユニット値を予期していましたが、%{actual} を取得しました。",
            ),
            ("ko", "단위 값을 예상했지만 %{actual}를 받았습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_definition_value",
        HashMap::from([
            ("en", "Expected a %{expected} value, but got %{actual}."),
            ("zh", "预期值为 %{expected}，但得到的是 %{actual}。"),
            (
                "de",
                "Es wurde ein %{expected}-Wert erwartet, aber %{actual} erhalten.",
            ),
            (
                "es",
                "Se esperaba un valor %{expected}, pero obtuve %{actual}.",
            ),
            (
                "fr",
                "Attendu une valeur %{expected}, mais j'ai obtenu %{actual}.",
            ),
            (
                "ja",
                "%{expected} 値を予期していましたが、%{actual} を取得しました。",
            ),
            ("ko", "%{expected} 값을 예상했지만 %{actual}를 얻었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_value_below_minimum",
        HashMap::from([
            (
                "en",
                "Value %{value} is less than the minimum allowed value of %{minimum}.",
            ),
            ("zh", "值 %{value} 小于 %{minimum} 的最小允许值。"),
            (
                "de",
                "Der Wert %{value} ist kleiner als der minimal zulässige Wert von %{minimum}.",
            ),
            (
                "es",
                "El valor %{value} es menor que el valor mínimo permitido de %{minimum}.",
            ),
            (
                "fr",
                "La valeur %{value} est inférieure à la valeur minimale autorisée de %{minimum}.",
            ),
            (
                "ja",
                "値 %{value} は、許容される最小値 %{minimum} より小さいです。",
            ),
            (
                "ko",
                "%{value} 값은 %{minimum}의 최소 허용 값보다 작습니다.",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_value_above_maximum",
        HashMap::from([
            (
                "en",
                "Value %{value} is greater than the maximum allowed value of %{maximum}.",
            ),
            ("zh", "值 %{value} 大于 %{maximum} 的最大允许值。"),
            (
                "de",
                "Der Wert %{value} ist größer als der maximal zulässige Wert von %{maximum}.",
            ),
            (
                "es",
                "El valor %{value} es mayor que el valor máximo permitido de %{maximum}.",
            ),
            (
                "fr",
                "La valeur %{value} est supérieure à la valeur maximale autorisée de %{maximum}.",
            ),
            (
                "ja",
                "値 %{value} は、許容最大値 %{maximum} を超えています。",
            ),
            ("ko", "%{value} 값은 %{maximum}의 최대 허용 값보다 큽니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unit_conversion_failed",
        HashMap::from([
            ("en", "Unit conversion failed: %{error}"),
            ("zh", "单位转换失败：%{error}"),
            ("de", "Einheitenumrechnung fehlgeschlagen: %{error}"),
            ("es", "Error en la conversión de unidades: %{error}"),
            ("fr", "Échec de la conversion d'unité : %{error}"),
            ("ja", "単位変換に失敗しました: %{error}"),
            ("ko", "단위 변환 실패: %{error}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_number_with_units_definition",
        HashMap::from([
            ("en", "Expected a number-with-units definition."),
            ("zh", "需要一个带有单位的数字定义。"),
            ("de", "Erwartet wurde eine Zahlen-mit-Einheiten-Definition."),
            ("es", "Se esperaba una definición de número con unidades."),
            (
                "fr",
                "Je m'attendais à une définition de nombre avec unités.",
            ),
            ("ja", "単位付きの数値定義が必要です。"),
            ("ko", "단위가 포함된 숫자 정의가 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_unknown_unit",
        HashMap::from([
            ("en", "Unknown unit '%{unit}'."),
            ("zh", "未知单位“%{unit}”。"),
            ("de", "Unbekannte Einheit „%{unit}“."),
            ("es", "Unidad desconocida '%{unit}'."),
            ("fr", "Unité inconnue '%{unit}'."),
            ("ja", "不明なユニット「%{unit}」。"),
            ("ko", "알 수 없는 단위 '%{unit}'."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_column_count_mismatch",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with %{actual} columns, but the current table expects %{expected} columns."),
            ("zh", "参数“%{parameter}”引用具有 %{actual} 列的表，但当前表需要 %{expected} 列。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle mit %{actual}-Spalten, aber die aktuelle Tabelle erwartet %{expected}-Spalten."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla con columnas %{actual}, pero la tabla actual espera columnas %{expected}."),
            ("fr", "Le paramètre « %{parameter} » fait référence à une table avec des colonnes %{actual}, mais la table actuelle attend des colonnes %{expected}."),
            ("ja", "パラメーター '%{parameter}' は %{actual} 列を持つテーブルを参照していますが、現在のテーブルは %{expected} 列を想定しています。"),
            ("ko", "매개변수 '%{parameter}'는 %{actual} 열이 있는 테이블을 참조하지만 현재 테이블에는 %{expected} 열이 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_missing_column_definition",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with no column definition at index %{index}."),
            ("zh", "参数“%{parameter}”引用索引 %{index} 处没有列定义的表。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle ohne Spaltendefinition am Index %{index}."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla sin definición de columna en el índice %{index}."),
            ("fr", "Le paramètre '%{parameter}' fait référence à une table sans définition de colonne à l'index %{index}."),
            ("ja", "パラメータ「%{parameter}」は、インデックス %{index} に列定義のないテーブルを参照しています。"),
            ("ko", "매개변수 '%{parameter}'는 인덱스 %{index}에서 컬럼 정의가 없는 테이블을 참조합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_value_below_minimum",
        HashMap::from([
            ("en", "Value %{value} in column '%{column}' is less than the minimum allowed value of %{minimum}."),
            ("zh", "“%{column}”列中的值 %{value} 小于 %{minimum} 的最小允许值。"),
            ("de", "Der Wert %{value} in der Spalte „%{column}“ ist kleiner als der minimal zulässige Wert von %{minimum}."),
            ("es", "El valor %{value} en la columna '%{column}' es menor que el valor mínimo permitido de %{minimum}."),
            ("fr", "La valeur %{value} dans la colonne « %{column} » est inférieure à la valeur minimale autorisée de %{minimum}."),
            ("ja", "列 '%{column}' の値 %{value} が、許容される最小値 %{minimum} 未満です。"),
            ("ko", "'%{column}' 열의 %{value} 값이 허용되는 최소 값인 %{minimum}보다 작습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_value_above_maximum",
        HashMap::from([
            ("en", "Value %{value} in column '%{column}' is greater than the maximum allowed value of %{maximum}."),
            ("zh", "“%{column}”列中的值 %{value} 大于 %{maximum} 的最大允许值。"),
            ("de", "Der Wert %{value} in der Spalte „%{column}“ ist größer als der maximal zulässige Wert von %{maximum}."),
            ("es", "El valor %{value} en la columna '%{column}' es mayor que el valor máximo permitido de %{maximum}."),
            ("fr", "La valeur %{value} dans la colonne « %{column} » est supérieure à la valeur maximale autorisée de %{maximum}."),
            ("ja", "列 '%{column}' の値 %{value} が、許容最大値 %{maximum} を超えています。"),
            ("ko", "'%{column}' 열의 %{value} 값이 %{maximum}의 최대 허용 값보다 큽니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_parameter",
        HashMap::from([
            ("en", "Parameter '%{parameter}' is expected to reference a table, but got %{actual}."),
            ("zh", "参数“%{parameter}”应引用表，但得到的是 %{actual}。"),
            ("de", "Der Parameter „%{parameter}“ soll auf eine Tabelle verweisen, hat aber %{actual}."),
            ("es", "Se espera que el parámetro '%{parameter}' haga referencia a una tabla, pero obtuvo %{actual}."),
            ("fr", "Le paramètre « %{parameter} » est censé faire référence à une table, mais a obtenu %{actual}."),
            ("ja", "パラメータ「%{parameter}」はテーブルを参照する必要がありますが、%{actual} を取得しました。"),
            ("ko", "'%{parameter}' 매개변수는 테이블을 참조해야 하지만 %{actual}를 가져왔습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_cell_missing_column_definition",
        HashMap::from([
            ("en", "No column definition exists for table cell at index %{index}."),
            ("zh", "索引 %{index} 处的表单元格不存在列定义。"),
            ("de", "Für die Tabellenzelle am Index %{index} ist keine Spaltendefinition vorhanden."),
            ("es", "No existe ninguna definición de columna para la celda de la tabla en el índice %{index}."),
            ("fr", "Aucune définition de colonne n'existe pour la cellule du tableau à l'index %{index}."),
            ("ja", "インデックス %{index} のテーブル セルには列定義が存在しません。"),
            ("ko", "인덱스 %{index}의 테이블 셀에 대한 컬럼 정의가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_expected_table_cell_number",
        HashMap::from([
            ("en", "Expected a numeric value for table cell, but got %{actual}."),
            ("zh", "表格单元格需要一个数值，但得到的是 %{actual}。"),
            ("de", "Es wurde ein numerischer Wert für die Tabellenzelle erwartet, aber %{actual} wurde angezeigt."),
            ("es", "Se esperaba un valor numérico para la celda de la tabla, pero obtuve %{actual}."),
            ("fr", "Je m'attendais à une valeur numérique pour la cellule du tableau, mais j'ai obtenu %{actual}."),
            ("ja", "表のセルには数値が必要ですが、%{actual} が返されました。"),
            ("ko", "테이블 셀에 숫자 값이 필요했지만 %{actual}가 발생했습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_unit_count_mismatch",
        HashMap::from([
            ("en", "Parameter '%{parameter}' references a table with %{actual} units for %{expected} columns."),
            ("zh", "参数“%{parameter}”引用 %{expected} 列的 %{actual} 单位的表。"),
            ("de", "Der Parameter „%{parameter}“ verweist auf eine Tabelle mit %{actual}-Einheiten für %{expected}-Spalten."),
            ("es", "El parámetro '%{parameter}' hace referencia a una tabla con unidades %{actual} para columnas %{expected}."),
            ("fr", "Le paramètre « %{parameter} » fait référence à une table avec des unités %{actual} pour les colonnes %{expected}."),
            ("ja", "パラメーター「%{parameter}」は、%{expected} 列の %{actual} 単位を持つテーブルを参照します。"),
            ("ko", "매개변수 '%{parameter}'는 %{expected} 열에 대해 %{actual} 단위가 있는 테이블을 참조합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_unit_conversion_failed",
        HashMap::from([
            ("en", "Cannot convert parameter '%{parameter}' column '%{column}' from %{source_unit} to %{target_unit}: %{error}"),
            ("zh", "无法将参数“%{parameter}”列“%{column}”从 %{source_unit} 转换为 %{target_unit}：%{error}"),
            ("de", "Der Parameter „%{parameter}“ und die Spalte „%{column}“ können nicht von %{source_unit} in %{target_unit} konvertiert werden: %{error}"),
            ("es", "No se puede convertir el parámetro '%{parameter}' columna '%{column}' de %{source_unit} a %{target_unit}: %{error}"),
            ("fr", "Impossible de convertir le paramètre « %{parameter} » de la colonne « %{column} » de %{source_unit} en %{target_unit} : %{error}"),
            ("ja", "パラメータ「%{parameter}」列「%{column}」を %{source_unit} から %{target_unit} に変換できません: %{error}"),
            ("ko", "매개변수 '%{parameter}' 열 '%{column}'을 %{source_unit}에서 %{target_unit}로 변환할 수 없습니다: %{error}"),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_table_cell_missing_unit",
        HashMap::from([
            ("en", "No unit exists for table cell at index %{index}."),
            ("zh", "索引 %{index} 处的表格单元格不存在单位。"),
            (
                "de",
                "Für die Tabellenzelle am Index %{index} ist keine Einheit vorhanden.",
            ),
            (
                "es",
                "No existe ninguna unidad para la celda de la tabla en el índice %{index}.",
            ),
            (
                "fr",
                "Aucune unité n'existe pour la cellule du tableau à l'index %{index}.",
            ),
            (
                "ja",
                "インデックス %{index} のテーブル セルにはユニットが存在しません。",
            ),
            ("ko", "인덱스 %{index}의 테이블 셀에 대한 단위가 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_sum_requires_numeric_arguments",
        HashMap::from([
            ("en", "sum() expects numeric arguments, got %{actual}."),
            ("zh", "sum() 需要数字参数，得到 %{actual}。"),
            (
                "de",
                "sum() erwartet numerische Argumente, erhält %{actual}.",
            ),
            ("es", "sum() espera argumentos numéricos, obtuvo %{actual}."),
            (
                "fr",
                "sum() attend des arguments numériques, j'ai obtenu %{actual}.",
            ),
            (
                "ja",
                "sum() は数値引数を必要とします。%{actual} を取得しました。",
            ),
            ("ko", "sum()에는 숫자 인수가 필요하며 %{actual}가 있습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_add_requires_numeric_arguments",
        HashMap::from([
            ("en", "add expects numeric arguments."),
            ("zh", "add 需要数字参数。"),
            ("de", "add erwartet numerische Argumente."),
            ("es", "add espera argumentos numéricos."),
            ("fr", "add attend des arguments numériques."),
            ("ja", "add は数値引数を必要とします。"),
            ("ko", "add에는 숫자 인수가 필요합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_finite",
        HashMap::from([
            ("en", "%{function} function argument must be finite."),
            ("zh", "%{function} 函数参数必须是有限的。"),
            ("de", "Das Funktionsargument %{function} muss endlich sein."),
            (
                "es",
                "El argumento de la función %{function} debe ser finito.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être fini.",
            ),
            ("ja", "%{function} 関数の引数は有限である必要があります。"),
            ("ko", "%{function} 함수 인수는 유한해야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_out_of_integer_range",
        HashMap::from([
            ("en", "%{function} function argument is out of range for an integer."),
            ("zh", "%{function} 函数参数超出整数范围。"),
            ("de", "Das Funktionsargument %{function} liegt außerhalb des gültigen Bereichs für eine Ganzzahl."),
            ("es", "El argumento de la función %{function} está fuera del rango para un número entero."),
            ("fr", "L’argument de la fonction %{function} est hors plage pour un nombre entier."),
            ("ja", "%{function} 関数の引数が整数の範囲外です。"),
            ("ko", "%{function} 함수 인수가 정수 범위를 벗어났습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_integer_conversion_failed",
        HashMap::from([
            ("en", "%{function} function argument could not be converted to an integer."),
            ("zh", "%{function} 函数参数无法转换为整数。"),
            ("de", "Das Funktionsargument %{function} konnte nicht in eine Ganzzahl konvertiert werden."),
            ("es", "El argumento de la función %{function} no se pudo convertir a un número entero."),
            ("fr", "L'argument de la fonction %{function} n'a pas pu être converti en nombre entier."),
            ("ja", "%{function} 関数の引数を整数に変換できませんでした。"),
            ("ko", "%{function} 함수 인수를 정수로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_float_precision_loss",
        HashMap::from([
            ("en", "%{function} function argument is too large to convert to a float without losing precision."),
            ("zh", "%{function} 函数参数太大，无法在不损失精度的情况下转换为浮点数。"),
            ("de", "Das Funktionsargument %{function} ist zu groß, um es ohne Präzisionsverlust in einen Gleitkommawert umzuwandeln."),
            ("es", "El argumento de la función %{function} es demasiado grande para convertirlo en flotante sin perder precisión."),
            ("fr", "L'argument de la fonction %{function} est trop volumineux pour être converti en flottant sans perte de précision."),
            ("ja", "%{function} 関数の引数が大きすぎるため、精度を失わずに float に変換できません。"),
            ("ko", "%{function} 함수 인수가 너무 커서 정밀도를 잃지 않고 부동 소수점으로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_float_conversion_failed",
        HashMap::from([
            ("en", "%{function} function argument could not be converted to a float."),
            ("zh", "%{function} 函数参数无法转换为浮点数。"),
            ("de", "Das Funktionsargument %{function} konnte nicht in eine Gleitkommazahl konvertiert werden."),
            ("es", "El argumento de la función %{function} no se pudo convertir en un valor flotante."),
            ("fr", "L'argument de la fonction %{function} n'a pas pu être converti en flottant."),
            ("ja", "%{function} 関数の引数を float に変換できませんでした。"),
            ("ko", "%{function} 함수 인수를 부동 소수점으로 변환할 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_missing_expected_argument",
        HashMap::from([
            (
                "en",
                "%{function} function is missing an expected argument.",
            ),
            ("zh", "%{function} 函数缺少预期参数。"),
            (
                "de",
                "Der Funktion %{function} fehlt ein erwartetes Argument.",
            ),
            (
                "es",
                "A la función %{function} le falta un argumento esperado.",
            ),
            (
                "fr",
                "La fonction %{function} ne dispose pas d'un argument attendu.",
            ),
            ("ja", "%{function} 関数に予期された引数がありません。"),
            ("ko", "%{function} 함수에 예상 인수가 누락되었습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_float",
        HashMap::from([
            ("en", "%{function} function argument must be a float."),
            ("zh", "%{function} 函数参数必须是浮点数。"),
            (
                "de",
                "Das Funktionsargument %{function} muss ein Float sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser un flotante.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être un flottant.",
            ),
            (
                "ja",
                "%{function} 関数の引数は浮動小数点でなければなりません。",
            ),
            ("ko", "%{function} 함수 인수는 부동 소수점이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_arguments_mixed_numeric_types",
        HashMap::from([
            ("en", "%{function} function arguments must all be the same numeric type."),
            ("zh", "%{function} 函数参数必须全部为相同的数字类型。"),
            ("de", "%{function}-Funktionsargumente müssen alle vom gleichen numerischen Typ sein."),
            ("es", "Todos los argumentos de la función %{function} deben ser del mismo tipo numérico."),
            ("fr", "Les arguments de la fonction %{function} doivent tous être du même type numérique."),
            ("ja", "%{function} 関数の引数はすべて同じ数値型である必要があります。"),
            ("ko", "%{function} 함수 인수는 모두 동일한 숫자 유형이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_number",
        HashMap::from([
            ("en", "%{function} function argument must be a number."),
            ("zh", "%{function} 函数参数必须是数字。"),
            (
                "de",
                "Das Funktionsargument %{function} muss eine Zahl sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser un número.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être un nombre.",
            ),
            ("ja", "%{function} 関数の引数は数値である必要があります。"),
            ("ko", "%{function} 함수 인수는 숫자여야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_clamp_minimum_exceeds_maximum",
        HashMap::from([
            ("en", "clamp function minimum argument must not be greater than its maximum argument."),
            ("zh", "钳位函数的最小参数不得大于其最大参数。"),
            ("de", "Das minimale Argument der Klemmfunktion darf nicht größer sein als das maximale Argument."),
            ("es", "El argumento mínimo de la función de sujeción no debe ser mayor que su argumento máximo."),
            ("fr", "L’argument minimum de la fonction clamp ne doit pas être supérieur à son argument maximum."),
            ("ja", "クランプ関数の最小引数は最大引数を超えてはなりません。"),
            ("ko", "클램프 함수의 최소 인수는 최대 인수보다 클 수 없습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_length_result_too_large",
        HashMap::from([
            ("en", "len function result is too large to fit in an integer."),
            ("zh", "len 函数结果太大，无法用整数表示。"),
            ("de", "Das Ergebnis der len-Funktion ist zu groß, um in eine ganze Zahl zu passen."),
            ("es", "El resultado de la función len es demasiado grande para caber en un número entero."),
            ("fr", "Le résultat de la fonction len est trop grand pour tenir dans un entier."),
            ("ja", "len 関数の結果が大きすぎて整数に収まりません。"),
            ("ko", "len 함수 결과가 너무 커서 정수에 맞지 않습니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_argument_must_be_string",
        HashMap::from([
            ("en", "%{function} function argument must be a string."),
            ("zh", "%{function} 函数参数必须是字符串。"),
            (
                "de",
                "Das Funktionsargument %{function} muss eine Zeichenfolge sein.",
            ),
            (
                "es",
                "El argumento de la función %{function} debe ser una cadena.",
            ),
            (
                "fr",
                "L’argument de la fonction %{function} doit être une chaîne.",
            ),
            ("ja", "%{function} 関数の引数は文字列である必要があります。"),
            ("ko", "%{function} 함수 인수는 문자열이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_function_if_condition_must_be_boolean",
        HashMap::from([
            ("en", "if function first argument must be a boolean."),
            ("zh", "if 函数第一个参数必须是布尔值。"),
            (
                "de",
                "Das erste Argument der Funktion if muss ein boolescher Wert sein.",
            ),
            (
                "es",
                "si el primer argumento de la función debe ser booleano.",
            ),
            (
                "fr",
                "si le premier argument de la fonction doit être un booléen.",
            ),
            ("ja", "if 関数の最初の引数はブール値でなければなりません。"),
            ("ko", "함수의 첫 번째 인수는 부울이어야 합니다."),
        ]),
    );
    translation_map.set_translation_key(
        "expression_engine_evaluation_custom_function_failed",
        HashMap::from([
            ("en", "Custom function failed: %{actual}"),
            ("zh", "自定义函数失败：%{actual}"),
            (
                "de",
                "Benutzerdefinierte Funktion fehlgeschlagen: %{actual}",
            ),
            ("es", "Error en la función personalizada: %{actual}"),
            ("fr", "Échec de la fonction personnalisée : %{actual}"),
            ("ja", "カスタム関数が失敗しました: %{actual}"),
            ("ko", "맞춤 기능 실패: %{actual}"),
        ]),
    );
}
