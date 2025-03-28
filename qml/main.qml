import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640
    color: palette.window

    MyObject {
        id: myObject
        number: 1
        string: myObject.transArgs("test_string", JSON.stringify({ number: myObject.number }))
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: myObject.transArgs("test_number_label", JSON.stringify({ number: myObject.number }))
            color: palette.text
        }

        Label {
            text: myObject.transArgs("test_string_label", JSON.stringify({ string: myObject.string }))
            color: palette.text
        }

        Button {
            text: myObject.trans("increment_number_button")

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: myObject.trans("say_hi_button")

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }

        Button {
            text: myObject.trans("quit_button")

            onClicked: Qt.quit()
        }
    }
}