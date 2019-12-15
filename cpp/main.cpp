
#include <QApplication>
#include <QWidget>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    QWidget w;
    w.setWindowTitle("lognplot");
    w.show();

    return app.exec();
}
