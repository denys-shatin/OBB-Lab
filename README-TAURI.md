# OBB Lab - Desktop App (Tauri)

Десктопная версия OBB Lab без браузерных ограничений.

## Преимущества десктопной версии

- **Нет браузерных хоткеев** - Ctrl+W, Ctrl+R, F5, F12 не закроют вкладку и не обновят страницу
- **Нет DevTools** - F12 и Ctrl+Shift+I отключены
- **Нативное окно** - работает как обычное приложение Windows
- **Меньше потребление памяти** - не нужен целый браузер

## Установка

### Готовый билд
Скачай `.msi` или `.exe` из [Releases](../../releases) и установи.

### Сборка из исходников

**Требования:**
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/)
- Visual Studio Build Tools с C++ компонентами

```bash
# Клонируй репо и переключись на tauri ветку
git clone https://github.com/denys-shatin/OBB-Lab.git
cd OBB-Lab
git checkout tauri

# Установи зависимости
npm install

# Запуск в режиме разработки
npm run tauri:dev

# Сборка релиза
npm run tauri:build
```

Готовый установщик будет в `src-tauri/target/release/bundle/`

## Управление

- **ЛКМ** - выделить куб/OBB
- **ПКМ** - вращение камеры
- **СКМ** - зум
- **WASD** - движение камеры (в режиме WASD)
- **Space/Shift** - вверх/вниз
- **Ctrl** - ускорение
- **T** - режим перемещения (Move)
- **R** - режим масштабирования (Scale)
- **Delete** - удалить выбранный OBB
- **Escape** - снять выделение
- **Ctrl+Z** - отмена

## Лицензия

MIT
