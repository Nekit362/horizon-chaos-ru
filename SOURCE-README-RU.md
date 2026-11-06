# Исходники отдельной сборки WYR

WyrHorizon/port/src — WYR; UpgraderHorizon/src — казино; WyrHorizon/fork — патчеры отдельного движка. Orbital Laser включён под MIT с LICENSE/NOTICE, нативным shim, помощниками и WGSL. В PROVENANCE.json зафиксированы upstream commits для повторного получения зависимостей. Полного исходного движка, JAR, декомпиляции, toolchain и target здесь нет.

Ресурсы WYR/Upgrader для личной сборки берутся из ваших оригинальных JAR:

    py -3.9 WyrHorizon/fork/combined_package.py --wyr-jar PATH/wyr.jar --upgrader-jar PATH/upgrader.jar --output PATH/wyr-full-content.hmod

Исходники патчеров сами по себе не устанавливаются в обычный Horizon: сначала нужен checkout закреплённого Horizon и horizon-mixins, native staging, затем fork/build_fork.py с --visual --orbital и сборка изменённого движка. Эта выдача не содержит готового .hnmod и не обещает совместимость со стандартным лаунчером.

Генератор build_port.py использует port/choice-definitions.json с описаниями и свойствами вариантов; оригинальный Java для пересборки не требуется. Сгенерированные native bundles и catalog.rs также включены. bootstrap.py получает закреплённые исходники Horizon/horizon-mixins, создаёт native stage и применяет fork. Запуск: `py -3.9 bootstrap.py` (получение исходников) или `py -3.9 bootstrap.py --build` (также полная release-сборка; нужны Git, Cargo, Rust и системный C/C++ toolchain). Он не запускает игру. Повторную сборку делайте с новым `--stage`, старые непустые папки не перезаписываются.

Некоторые прежние диагностические/preview scripts сохраняют локальные пути к инструментам; они не входят в bootstrap flow. combined_package.py использует относительное расположение обоих проектов и принимает оригинальные JAR через параметры. Совместимость и фактическую проверку конкретной выдачи смотрите в сопровождающем отчёте: наличие скрипта пересборки не означает, что выполнен интерактивный тест игры.

Содержимое .hmod включает оригинальные ресурсы двух модов для личного использования. Публичная публикация этих ресурсов в этом проекте не выполнялась.
