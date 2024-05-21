import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 2.12
import QtQuick.Window 2.12

import rs.chir.rachat 1.0

ApplicationWindow {
    height: 480
    title: qsTr("Select Homeserver — Rachat")
    visible: true
    width: 640

    SelectHomeserver {
        id: selectHomeserver
        errorString: "Please enter a homeserver name."
    }

    Label {
        id: welcomeLabel
        text: qsTr("Welcome to Rachat!")
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        font.pixelSize: 48
        wrapMode: Text.Wrap
        padding: 16
    }
    Label {
        id: serverEntryTitleLabel
        text: qsTr("Before you can start chatting, you need to select which server you are using.")
        anchors.top: welcomeLabel.bottom
        wrapMode: Text.Wrap
        padding: 8
    }
    Label {
        id: serverEntryDescriptionLabel
        text: qsTr("Server name:")
        anchors.bottom: serverField.bottom
        anchors.left: parent.left
        padding: 8
    }
    TextField {
        id: serverField
        placeholderText: 'matrix.example'
        anchors.left: serverEntryDescriptionLabel.right
        anchors.top: serverEntryTitleLabel.bottom
        padding: 8
        onTextChanged: selectHomeserver.onHomeserverTextChanged(serverField.text)
    }
    Button {
        id: selectButton
        text: qsTr("Select")
        anchors.left: parent.left
        anchors.top: serverField.bottom
        onClicked: selectHomeserver.selectHomeserver(serverField.text)
        padding: 8
        enabled: selectHomeserver.errorString.length == 0
    }
    Label {
        text: selectHomeserver.errorString
        visible: selectHomeserver.errorString.length > 0
        anchors.left: selectButton.right
        anchors.bottom: selectButton.bottom
        padding: 8
        wrapMode: Text.Wrap
    }
}
