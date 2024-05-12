import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import rs.chir.rachat 1.0

ApplicationWindow {
    height: 480
    title: qsTr("%1 — Rachat").arg(rootWindow.titleString)
    visible: true
    width: 640

    RootWindow {
        id: rootWindow
        titleString: qsTr("Select Profile")
    }
}
