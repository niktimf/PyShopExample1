

///Задание 2. Разработать тесты для функции определения счета в игре
/// Задача:
/// Для разработанной в предыдущем задании функции getScore(game_stamps, offset) разработайте unit-тесты.
/// Тесты должны учитывать все возможные случаи использования функции,
/// концентрироваться на проверке одного случая, не повторяться, название тестов должно отражать суть выполняемой проверки.



#[cfg(test)]
mod tests {
    use crate::{generate_game, get_score, INITIAL_STAMP, Score, Stamp};
    use super::*;

    //Пустой массив game_stamps
    #[test]
    fn test_empty_game_stamps() {
        let game_stamps = vec![];
        let offset = 10;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (0, 0));
    }

    //Значение offset меньше чем у первого элемента массива
    #[test]
    fn test_offset_less_than_first_stamp() {
        let game_stamps = vec![Stamp { offset: 10, score: Score { home: 1, away: 0 } }];
        let offset = 5;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (0, 0));
    }

    //offset, который совпадает с одним из элементов game_stamps
    #[test]
    fn test_exact_match_offset() {
        let game_stamps = vec![Stamp { offset: 10, score: Score { home: 1, away: 0 } }];
        let offset = 10;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (1, 0));
    }

    //offset, который находится между двумя offset в game_stamps
    #[test]
    fn test_offset_between_stamps() {
        let game_stamps = vec![
            Stamp { offset: 10, score: Score { home: 1, away: 0 } },
            Stamp { offset: 20, score: Score { home: 2, away: 0 } }
        ];
        let offset = 15;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (1, 0));
    }

    //offset, который больше, чем у последнего элемента game_stamps
    #[test]
    fn test_offset_greater_than_last_stamp() {
        let game_stamps = vec![
            Stamp { offset: 10, score: Score { home: 1, away: 0 } },
            Stamp { offset: 20, score: Score { home: 2, away: 0 } }
        ];
        let offset = 25;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (2, 0));
    }

    //Равный счет
    #[test]
    fn test_equal_score() {
        let game_stamps = vec![
            Stamp { offset: 10, score: Score { home: 1, away: 1 } },
            Stamp { offset: 20, score: Score { home: 2, away: 2 } }
        ];
        let offset = 15;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (1, 1));
    }

    //Пустой массив, но с большим смещением
    #[test]
    fn test_empty_game_stamps_large_offset() {
        let game_stamps = vec![];
        let offset = 1000;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (0, 0));
    }

    //Случай с массивом, содержащим только начальную метку
    #[test]
    fn test_only_initial_stamp() {
        let game_stamps = vec![INITIAL_STAMP];
        let offset = 1;
        let result = get_score(&game_stamps, offset);
        assert_eq!(result, (0, 0));
    }

    //Проверка порядка счета
    #[test]
    fn test_score_order() {
        let game = generate_game();
        for stamp in &game {
            assert!(stamp.score.home >= 0);
            assert!(stamp.score.away >= 0);
        }
    }

    //Отрицательное смещение
    #[test]
    fn test_negative_offset() {
        let game = generate_game();
        let result = get_score(&game, -5);
        assert_eq!(result, (0, 0));
    }

}