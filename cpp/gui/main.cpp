
#include <QApplication>
#include <QWidget>

#include "clognplot.h"

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    QWidget w;
    w.setWindowTitle("lognplot");
    w.show();

    // x();

    return app.exec();
}
