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
    }

    Label {
        id: welcomeLabel
        text: qsTr("Welcome to Rachat!")
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        font.pixelSize: 48
        wrapMode: Text.Wrap
    }
    Label {
        id: serverEntryTitleLabel
        text: qsTr("Before you can start chatting, you need to select which server you are using.")
        anchors.top: welcomeLabel.bottom
        wrapMode: Text.Wrap
    }
    Label {
        id: serverEntryDescriptionLabel
        text: qsTr("Server name:")
        anchors.top: serverEntryTitleLabel.bottom
        anchors.left: parent.left
    }
    TextField {
        id: serverField
        placeholderText: 'matrix.example'
        anchors.left: serverEntryDescriptionLabel.right
        anchors.top: serverEntryDescriptionLabel.top
    }
    Button {
        id: selectButton
        text: qsTr("Select")
        anchors.horizontalCenter: serverField.left
        anchors.top: serverField.bottom
        onClicked: selectHomeserver.selectHomeserver(serverField.text)
    }
}
