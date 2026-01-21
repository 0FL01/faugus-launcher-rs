// Internationalization (i18n) module
// Handles translations and localization

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::paths::Paths;

/// Internationalization manager
pub struct I18n {
    language: String,
    translations: HashMap<String, String>,
}

impl I18n {
    /// Create a new i18n instance
    pub fn new(language: String) -> Self {
        let translations = Self::load_translations(&language);

        Self {
            language,
            translations,
        }
    }

    /// Load translations for the specified language
    fn load_translations(language: &str) -> HashMap<String, String> {
        let mut translations = HashMap::new();

        // Try to load from system locale directory
        let _locale_paths = [
            Self::get_locale_dir(format!("faugus-launcher-{}", language)),
            Self::get_locale_dir(format!("faugus-launcher_{}", language)),
            Self::get_locale_dir(format!(
                "locales/{}/LC_MESSAGES/faugus-launcher.mo",
                language
            )),
        ];

        // For now, use built-in translations
        // In production, you would load .mo files or use gettext
        translations.extend(Self::get_builtin_translations(language));

        translations
    }

    /// Get locale directory path
    fn get_locale_dir(relative: String) -> PathBuf {
        // Check multiple possible locations
        let possible_paths = vec![
            Paths::system_data(&format!("locale/{}", &relative)),
            Some(PathBuf::from("/usr/share/locale").join(&relative)),
            Some(PathBuf::from("/usr/local/share/locale").join(&relative)),
        ];

        for p in possible_paths.into_iter().flatten() {
            if p.exists() {
                return p;
            }
        }

        // Fallback
        Paths::user_data(&format!("locale/{}", &relative))
    }

    /// Get built-in translations (fallback)
    fn get_builtin_translations(language: &str) -> HashMap<String, String> {
        let mut translations = HashMap::new();

        // English (default)
        let en_translations = vec![
            ("Faugus Launcher", "Faugus Launcher"),
            ("Play", "Play"),
            ("Stop", "Stop"),
            ("Retry", "Retry"),
            ("Running", "Running"),
            ("Launching...", "Launching..."),
            ("Add", "Add"),
            ("Edit", "Edit"),
            ("Delete", "Delete"),
            ("Settings", "Settings"),
            ("Game", "Game"),
            ("Games", "Games"),
            ("Title", "Title"),
            ("Path", "Path"),
            ("Prefix", "Prefix"),
            ("Runner", "Runner"),
            ("Launch Options", "Launch Options"),
            ("MangoHud", "MangoHud"),
            ("GameMode", "GameMode"),
            ("Duplicate", "Duplicate"),
            ("Hide", "Hide"),
            ("Show", "Show"),
            ("Open game location", "Open game location"),
            ("Open prefix location", "Open prefix location"),
            ("Show logs", "Show logs"),
            ("Close", "Close"),
            ("Cancel", "Cancel"),
            ("Apply", "Apply"),
            ("OK", "OK"),
            ("Yes", "Yes"),
            ("No", "No"),
            ("Warning", "Warning"),
            ("Error", "Error"),
            ("Information", "Information"),
            ("Confirm", "Confirm"),
            ("Are you sure?", "Are you sure?"),
            (
                "This action cannot be undone.",
                "This action cannot be undone.",
            ),
            ("Game deleted successfully", "Game deleted successfully"),
            ("Game saved successfully", "Game saved successfully"),
            ("Failed to save game", "Failed to save game"),
            ("Failed to delete game", "Failed to delete game"),
            ("No game selected", "No game selected"),
            ("Please select a game first", "Please select a game first"),
            ("Search games...", "Search games..."),
            ("Interface Mode", "Interface Mode"),
            ("List", "List"),
            ("Blocks", "Blocks"),
            ("Banners", "Banners"),
            ("Language", "Language"),
            ("Theme", "Theme"),
            ("Dark Theme", "Dark Theme"),
            ("System Tray", "System Tray"),
            ("Start at Boot", "Start at Boot"),
            ("Start Maximized", "Start Maximized"),
            ("Start Fullscreen", "Start Fullscreen"),
            ("Close on Launch", "Close on Launch"),
            ("Enable Logging", "Enable Logging"),
            ("Show Hidden Games", "Show Hidden Games"),
        ];

        match language {
            "es" => {
                // Spanish translations
                translations.insert("Faugus Launcher".into(), "Faugus Launcher".into());
                translations.insert("Play".into(), "Jugar".into());
                translations.insert("Stop".into(), "Detener".into());
                translations.insert("Retry".into(), "Reintentar".into());
                translations.insert("Running".into(), "Ejecutándose".into());
                translations.insert("Launching...".into(), "Lanzando...".into());
                translations.insert("Add".into(), "Añadir".into());
                translations.insert("Edit".into(), "Editar".into());
                translations.insert("Delete".into(), "Eliminar".into());
                translations.insert("Settings".into(), "Configuración".into());
                translations.insert("Game".into(), "Juego".into());
                translations.insert("Games".into(), "Juegos".into());
                translations.insert("Title".into(), "Título".into());
                translations.insert("Path".into(), "Ruta".into());
                translations.insert("Prefix".into(), "Prefijo".into());
                translations.insert("Runner".into(), "Runner".into());
                translations.insert("Launch Options".into(), "Opciones de Lanzamiento".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Duplicar".into());
                translations.insert("Hide".into(), "Ocultar".into());
                translations.insert("Show".into(), "Mostrar".into());
                translations.insert(
                    "Open game location".into(),
                    "Abrir ubicación del juego".into(),
                );
                translations.insert(
                    "Open prefix location".into(),
                    "Abrir ubicación del prefijo".into(),
                );
                translations.insert("Show logs".into(), "Mostrar registros".into());
                translations.insert("Close".into(), "Cerrar".into());
                translations.insert("Cancel".into(), "Cancelar".into());
                translations.insert("Apply".into(), "Aplicar".into());
                translations.insert("OK".into(), "Aceptar".into());
                translations.insert("Yes".into(), "Sí".into());
                translations.insert("No".into(), "No".into());
                translations.insert("Warning".into(), "Advertencia".into());
                translations.insert("Error".into(), "Error".into());
                translations.insert("Information".into(), "Información".into());
                translations.insert("Confirm".into(), "Confirmar".into());
                translations.insert("Are you sure?".into(), "¿Estás seguro?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Esta acción no se puede deshacer.".into(),
                );
                translations.insert(
                    "Game deleted successfully".into(),
                    "Juego eliminado correctamente".into(),
                );
                translations.insert(
                    "Game saved successfully".into(),
                    "Juego guardado correctamente".into(),
                );
                translations.insert(
                    "Failed to save game".into(),
                    "Error al guardar el juego".into(),
                );
                translations.insert(
                    "Failed to delete game".into(),
                    "Error al eliminar el juego".into(),
                );
                translations.insert(
                    "No game selected".into(),
                    "Ningún juego seleccionado".into(),
                );
                translations.insert(
                    "Please select a game first".into(),
                    "Por favor, selecciona un juego primero".into(),
                );
                translations.insert("Search games...".into(), "Buscar juegos...".into());
                translations.insert("Interface Mode".into(), "Modo de Interfaz".into());
                translations.insert("List".into(), "Lista".into());
                translations.insert("Blocks".into(), "Bloques".into());
                translations.insert("Banners".into(), "Banners".into());
                translations.insert("Language".into(), "Idioma".into());
                translations.insert("Theme".into(), "Tema".into());
                translations.insert("Dark Theme".into(), "Tema Oscuro".into());
                translations.insert("System Tray".into(), "Bandeja del Sistema".into());
                translations.insert("Start at Boot".into(), "Iniciar al Arrancar".into());
                translations.insert("Start Maximized".into(), "Iniciar Maximizado".into());
                translations.insert(
                    "Start Fullscreen".into(),
                    "Iniciar en Pantalla Completa".into(),
                );
                translations.insert("Close on Launch".into(), "Cerrar al Lanzar".into());
                translations.insert("Enable Logging".into(), "Habilitar Registro".into());
                translations.insert("Show Hidden Games".into(), "Mostrar Juegos Ocultos".into());
            }
            "fr" => {
                // French translations
                translations.insert("Play".into(), "Jouer".into());
                translations.insert("Add".into(), "Ajouter".into());
                translations.insert("Edit".into(), "Modifier".into());
                translations.insert("Delete".into(), "Supprimer".into());
                translations.insert("Settings".into(), "Paramètres".into());
                translations.insert("Game".into(), "Jeu".into());
                translations.insert("Games".into(), "Jeux".into());
                translations.insert("Title".into(), "Titre".into());
                translations.insert("Path".into(), "Chemin".into());
                translations.insert("Prefix".into(), "Préfixe".into());
                translations.insert("Runner".into(), "Runner".into());
                translations.insert("Launch Options".into(), "Options de Lancement".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Dupliquer".into());
                translations.insert("Hide".into(), "Masquer".into());
                translations.insert("Show".into(), "Afficher".into());
                translations.insert(
                    "Open game location".into(),
                    "Ouvrir l'emplacement du jeu".into(),
                );
                translations.insert(
                    "Open prefix location".into(),
                    "Ouvrir l'emplacement du préfixe".into(),
                );
                translations.insert("Show logs".into(), "Afficher les journaux".into());
                translations.insert("Close".into(), "Fermer".into());
                translations.insert("Cancel".into(), "Annuler".into());
                translations.insert("Apply".into(), "Appliquer".into());
                translations.insert("OK".into(), "OK".into());
                translations.insert("Yes".into(), "Oui".into());
                translations.insert("No".into(), "Non".into());
                translations.insert("Warning".into(), "Avertissement".into());
                translations.insert("Error".into(), "Erreur".into());
                translations.insert("Information".into(), "Information".into());
                translations.insert("Confirm".into(), "Confirmer".into());
                translations.insert("Are you sure?".into(), "Êtes-vous sûr?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Cette action ne peut pas être annulée.".into(),
                );
            }
            "de" => {
                // German translations
                translations.insert("Play".into(), "Spielen".into());
                translations.insert("Add".into(), "Hinzufügen".into());
                translations.insert("Edit".into(), "Bearbeiten".into());
                translations.insert("Delete".into(), "Löschen".into());
                translations.insert("Settings".into(), "Einstellungen".into());
                translations.insert("Game".into(), "Spiel".into());
                translations.insert("Games".into(), "Spiele".into());
                translations.insert("Title".into(), "Titel".into());
                translations.insert("Path".into(), "Pfad".into());
                translations.insert("Prefix".into(), "Präfix".into());
                translations.insert("Runner".into(), "Runner".into());
                translations.insert("Launch Options".into(), "Startoptionen".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Duplizieren".into());
                translations.insert("Hide".into(), "Ausblenden".into());
                translations.insert("Show".into(), "Anzeigen".into());
                translations.insert("Open game location".into(), "Spielstandort öffnen".into());
                translations.insert(
                    "Open prefix location".into(),
                    "Präfixstandort öffnen".into(),
                );
                translations.insert("Show logs".into(), "Protokolle anzeigen".into());
                translations.insert("Close".into(), "Schließen".into());
                translations.insert("Cancel".into(), "Abbrechen".into());
                translations.insert("Apply".into(), "Anwenden".into());
                translations.insert("OK".into(), "OK".into());
                translations.insert("Yes".into(), "Ja".into());
                translations.insert("No".into(), "Nein".into());
                translations.insert("Warning".into(), "Warnung".into());
                translations.insert("Error".into(), "Fehler".into());
                translations.insert("Information".into(), "Information".into());
                translations.insert("Confirm".into(), "Bestätigen".into());
                translations.insert("Are you sure?".into(), "Sind Sie sicher?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Diese Aktion kann nicht rückgängig gemacht werden.".into(),
                );
            }
            "pt_BR" | "pt" => {
                // Portuguese translations
                translations.insert("Play".into(), "Jogar".into());
                translations.insert("Add".into(), "Adicionar".into());
                translations.insert("Edit".into(), "Editar".into());
                translations.insert("Delete".into(), "Excluir".into());
                translations.insert("Settings".into(), "Configurações".into());
                translations.insert("Game".into(), "Jogo".into());
                translations.insert("Games".into(), "Jogos".into());
                translations.insert("Title".into(), "Título".into());
                translations.insert("Path".into(), "Caminho".into());
                translations.insert("Prefix".into(), "Prefixo".into());
                translations.insert("Runner".into(), "Runner".into());
                translations.insert("Launch Options".into(), "Opções de Lançamento".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Duplicar".into());
                translations.insert("Hide".into(), "Ocultar".into());
                translations.insert("Show".into(), "Mostrar".into());
                translations.insert(
                    "Open game location".into(),
                    "Abrir localização do jogo".into(),
                );
                translations.insert(
                    "Open prefix location".into(),
                    "Abrir localização do prefixo".into(),
                );
                translations.insert("Show logs".into(), "Mostrar logs".into());
                translations.insert("Close".into(), "Fechar".into());
                translations.insert("Cancel".into(), "Cancelar".into());
                translations.insert("Apply".into(), "Aplicar".into());
                translations.insert("OK".into(), "OK".into());
                translations.insert("Yes".into(), "Sim".into());
                translations.insert("No".into(), "Não".into());
                translations.insert("Warning".into(), "Aviso".into());
                translations.insert("Error".into(), "Erro".into());
                translations.insert("Information".into(), "Informação".into());
                translations.insert("Confirm".into(), "Confirmar".into());
                translations.insert("Are you sure?".into(), "Tem certeza?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Esta ação não pode ser desfeita.".into(),
                );
            }
            "ru" => {
                // Russian translations
                translations.insert("Play".into(), "Играть".into());
                translations.insert("Add".into(), "Добавить".into());
                translations.insert("Edit".into(), "Редактировать".into());
                translations.insert("Delete".into(), "Удалить".into());
                translations.insert("Settings".into(), "Настройки".into());
                translations.insert("Game".into(), "Игра".into());
                translations.insert("Games".into(), "Игры".into());
                translations.insert("Title".into(), "Название".into());
                translations.insert("Path".into(), "Путь".into());
                translations.insert("Prefix".into(), "Префикс".into());
                translations.insert("Runner".into(), "Раннер".into());
                translations.insert("Launch Options".into(), "Параметры запуска".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Дублировать".into());
                translations.insert("Hide".into(), "Скрыть".into());
                translations.insert("Show".into(), "Показать".into());
                translations.insert(
                    "Open game location".into(),
                    "Открыть расположение игры".into(),
                );
                translations.insert(
                    "Open prefix location".into(),
                    "Открыть расположение префикса".into(),
                );
                translations.insert("Show logs".into(), "Показать журналы".into());
                translations.insert("Close".into(), "Закрыть".into());
                translations.insert("Cancel".into(), "Отмена".into());
                translations.insert("Apply".into(), "Применить".into());
                translations.insert("OK".into(), "ОК".into());
                translations.insert("Yes".into(), "Да".into());
                translations.insert("No".into(), "Нет".into());
                translations.insert("Warning".into(), "Предупреждение".into());
                translations.insert("Error".into(), "Ошибка".into());
                translations.insert("Information".into(), "Информация".into());
                translations.insert("Confirm".into(), "Подтвердить".into());
                translations.insert("Are you sure?".into(), "Вы уверены?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Это действие нельзя отменить.".into(),
                );
            }
            "zh_CN" | "zh" => {
                // Chinese translations
                translations.insert("Play".into(), "播放".into());
                translations.insert("Add".into(), "添加".into());
                translations.insert("Edit".into(), "编辑".into());
                translations.insert("Delete".into(), "删除".into());
                translations.insert("Settings".into(), "设置".into());
                translations.insert("Game".into(), "游戏".into());
                translations.insert("Games".into(), "游戏".into());
                translations.insert("Title".into(), "标题".into());
                translations.insert("Path".into(), "路径".into());
                translations.insert("Prefix".into(), "前缀".into());
                translations.insert("Runner".into(), "运行器".into());
                translations.insert("Launch Options".into(), "启动选项".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "复制".into());
                translations.insert("Hide".into(), "隐藏".into());
                translations.insert("Show".into(), "显示".into());
                translations.insert("Open game location".into(), "打开游戏位置".into());
                translations.insert("Open prefix location".into(), "打开前缀位置".into());
                translations.insert("Show logs".into(), "显示日志".into());
                translations.insert("Close".into(), "关闭".into());
                translations.insert("Cancel".into(), "取消".into());
                translations.insert("Apply".into(), "应用".into());
                translations.insert("OK".into(), "确定".into());
                translations.insert("Yes".into(), "是".into());
                translations.insert("No".into(), "否".into());
                translations.insert("Warning".into(), "警告".into());
                translations.insert("Error".into(), "错误".into());
                translations.insert("Information".into(), "信息".into());
                translations.insert("Confirm".into(), "确认".into());
                translations.insert("Are you sure?".into(), "您确定吗？".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "此操作无法撤销。".into(),
                );
            }
            "sv" => {
                // Swedish translations
                translations.insert("Faugus Launcher".into(), "Faugus Launcher".into());
                translations.insert("Play".into(), "Spela".into());
                translations.insert("Stop".into(), "Stoppa".into());
                translations.insert("Retry".into(), "Försök igen".into());
                translations.insert("Running".into(), "Körs".into());
                translations.insert("Launching...".into(), "Startar...".into());
                translations.insert("Add".into(), "Lägg till".into());
                translations.insert("Edit".into(), "Redigera".into());
                translations.insert("Delete".into(), "Ta bort".into());
                translations.insert("Settings".into(), "Inställningar".into());
                translations.insert("Game".into(), "Spel".into());
                translations.insert("Games".into(), "Spel".into());
                translations.insert("Title".into(), "Titel".into());
                translations.insert("Path".into(), "Sökväg".into());
                translations.insert("Prefix".into(), "Prefix".into());
                translations.insert("Runner".into(), "Runner".into());
                translations.insert("Launch Options".into(), "Startalternativ".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Duplicera".into());
                translations.insert("Hide".into(), "Dölj".into());
                translations.insert("Show".into(), "Visa".into());
                translations.insert("Open game location".into(), "Öppna spelplats".into());
                translations.insert("Open prefix location".into(), "Öppna prefix-plats".into());
                translations.insert("Show logs".into(), "Visa loggar".into());
                translations.insert("Close".into(), "Stäng".into());
                translations.insert("Cancel".into(), "Avbryt".into());
                translations.insert("Apply".into(), "Verkställ".into());
                translations.insert("OK".into(), "OK".into());
                translations.insert("Yes".into(), "Ja".into());
                translations.insert("No".into(), "Nej".into());
                translations.insert("Warning".into(), "Varning".into());
                translations.insert("Error".into(), "Fel".into());
                translations.insert("Information".into(), "Information".into());
                translations.insert("Confirm".into(), "Bekräfta".into());
                translations.insert("Are you sure?".into(), "Är du säker?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Denna åtgärd kan inte ångras.".into(),
                );
                translations.insert("Game deleted successfully".into(), "Spel borttaget".into());
                translations.insert("Game saved successfully".into(), "Spel sparat".into());
                translations.insert("Failed to save game".into(), "Kunde inte spara spel".into());
                translations.insert(
                    "Failed to delete game".into(),
                    "Kunde inte ta bort spel".into(),
                );
                translations.insert("No game selected".into(), "Inget spel valt".into());
                translations.insert(
                    "Please select a game first".into(),
                    "Välj ett spel först".into(),
                );
                translations.insert("Search games...".into(), "Sök spel...".into());
                translations.insert("Interface Mode".into(), "Gränssnittsläge".into());
                translations.insert("List".into(), "Lista".into());
                translations.insert("Blocks".into(), "Block".into());
                translations.insert("Banners".into(), "Banners".into());
                translations.insert("Language".into(), "Språk".into());
                translations.insert("Theme".into(), "Tema".into());
                translations.insert("Dark Theme".into(), "Mörkt tema".into());
                translations.insert("System Tray".into(), "Systemfält".into());
                translations.insert("Start at Boot".into(), "Starta vid uppstart".into());
                translations.insert("Start Maximized".into(), "Starta maximerad".into());
                translations.insert("Start Fullscreen".into(), "Starta helskärm".into());
                translations.insert("Close on Launch".into(), "Stäng vid start".into());
                translations.insert("Enable Logging".into(), "Aktivera loggning".into());
                translations.insert("Show Hidden Games".into(), "Visa dolda spel".into());
            }
            "uk" => {
                // Ukrainian translations
                translations.insert("Faugus Launcher".into(), "Faugus Launcher".into());
                translations.insert("Play".into(), "Грати".into());
                translations.insert("Stop".into(), "Зупинити".into());
                translations.insert("Retry".into(), "Повторити".into());
                translations.insert("Running".into(), "Запущено".into());
                translations.insert("Launching...".into(), "Запуск...".into());
                translations.insert("Add".into(), "Додати".into());
                translations.insert("Edit".into(), "Редагувати".into());
                translations.insert("Delete".into(), "Видалити".into());
                translations.insert("Settings".into(), "Налаштування".into());
                translations.insert("Game".into(), "Гра".into());
                translations.insert("Games".into(), "Ігри".into());
                translations.insert("Title".into(), "Назва".into());
                translations.insert("Path".into(), "Шлях".into());
                translations.insert("Prefix".into(), "Префікс".into());
                translations.insert("Runner".into(), "Раннер".into());
                translations.insert("Launch Options".into(), "Параметри запуску".into());
                translations.insert("MangoHud".into(), "MangoHud".into());
                translations.insert("GameMode".into(), "GameMode".into());
                translations.insert("Duplicate".into(), "Дублювати".into());
                translations.insert("Hide".into(), "Сховати".into());
                translations.insert("Show".into(), "Показати".into());
                translations.insert(
                    "Open game location".into(),
                    "Відкрити розташування гри".into(),
                );
                translations.insert(
                    "Open prefix location".into(),
                    "Відкрити розташування префікса".into(),
                );
                translations.insert("Show logs".into(), "Показати журнали".into());
                translations.insert("Close".into(), "Закрити".into());
                translations.insert("Cancel".into(), "Скасувати".into());
                translations.insert("Apply".into(), "Застосувати".into());
                translations.insert("OK".into(), "ОК".into());
                translations.insert("Yes".into(), "Так".into());
                translations.insert("No".into(), "Ні".into());
                translations.insert("Warning".into(), "Попередження".into());
                translations.insert("Error".into(), "Помилка".into());
                translations.insert("Information".into(), "Інформація".into());
                translations.insert("Confirm".into(), "Підтвердити".into());
                translations.insert("Are you sure?".into(), "Ви впевнені?".into());
                translations.insert(
                    "This action cannot be undone.".into(),
                    "Цю дію неможливо скасувати.".into(),
                );
                translations.insert(
                    "Game deleted successfully".into(),
                    "Гру успішно видалено".into(),
                );
                translations.insert(
                    "Game saved successfully".into(),
                    "Гру успішно збережено".into(),
                );
                translations.insert(
                    "Failed to save game".into(),
                    "Не вдалося зберегти гру".into(),
                );
                translations.insert(
                    "Failed to delete game".into(),
                    "Не вдалося видалити гру".into(),
                );
                translations.insert("No game selected".into(), "Гру не вибрано".into());
                translations.insert(
                    "Please select a game first".into(),
                    "Будь ласка, спочатку виберіть гру".into(),
                );
                translations.insert("Search games...".into(), "Пошук ігор...".into());
                translations.insert("Interface Mode".into(), "Режим інтерфейсу".into());
                translations.insert("List".into(), "Список".into());
                translations.insert("Blocks".into(), "Блоки".into());
                translations.insert("Banners".into(), "Банери".into());
                translations.insert("Language".into(), "Мова".into());
                translations.insert("Theme".into(), "Тема".into());
                translations.insert("Dark Theme".into(), "Темна тема".into());
                translations.insert("System Tray".into(), "Системний лоток".into());
                translations.insert("Start at Boot".into(), "Запуск при завантаженні".into());
                translations.insert("Start Maximized".into(), "Запуск максимізованим".into());
                translations.insert("Start Fullscreen".into(), "Запуск на весь екран".into());
                translations.insert("Close on Launch".into(), "Закрити при запуску".into());
                translations.insert("Enable Logging".into(), "Увімкнути журналювання".into());
                translations.insert("Show Hidden Games".into(), "Показати приховані ігри".into());
            }
            _ => {
                // Default to English
                for (key, value) in &en_translations {
                    translations.insert(key.to_string(), value.to_string());
                }
            }
        }

        // If not English, fill in missing translations with English
        if language != "en" && language != "en_US" {
            for (key, value) in &en_translations {
                if !translations.contains_key(*key) {
                    translations.insert(key.to_string(), value.to_string());
                }
            }
        }

        translations
    }

    /// Translate a string
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Get current language
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Set language and reload translations
    pub fn set_language(&mut self, language: String) {
        self.language = language.clone();
        self.translations = Self::load_translations(&language);
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new("en_US".to_string())
    }
}

/// Get system locale
/// DEPRECATED: Use detect_language() or I18n::from_system_locale()
#[allow(dead_code)]
pub fn get_system_locale() -> String {
    // Check environment variables
    if let Ok(lang) = std::env::var("LANG") {
        if let Some(locale) = lang.split('.').next() {
            return locale.to_string();
        }
    }

    if let Ok(lang) = std::env::var("LC_MESSAGES") {
        if let Some(locale) = lang.split('.').next() {
            return locale.to_string();
        }
    }

    // Default to en_US
    "en_US".to_string()
}

/// Detect language from config or system
/// TODO: Use for first-run language detection
#[allow(dead_code)]
pub fn detect_language(config_language: Option<String>) -> String {
    if let Some(lang) = config_language {
        if !lang.is_empty() {
            return lang;
        }
    }

    get_system_locale()
}
