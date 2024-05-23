import QtQuick 2.12
import QtQuick.Controls 2.12

import rs.chir.rachat 1.0

Item {
    LoginWindow {
        id: loginWindow
    }
    Label {
        id: loginTitle
        text: qsTr("Login to Matrix")
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        font.pixelSize: 48
        padding: 16
    }
    Label {
        id: loginDescription
        text: qsTr("Enter your username and password for %1.").arg(loginWindow.homeserver)
        anchors.bottom: goBackButton.bottom
        anchors.left: parent.left
        padding: 8
    }
    Button {
        id: goBackButton
        text: qsTr("Not your homeserver?")
        anchors.top: loginTitle.bottom
        anchors.left: loginDescription.right
        padding: 8
    }
    Label {
        id: usernameLabel
        text: qsTr("Username:")
        anchors.bottom: usernameTextField.bottom
        anchors.left: parent.left
        padding: 8
    }
    TextField {
        id: usernameTextField
        anchors.top: loginDescription.bottom
        anchors.left: usernameLabel.right
        padding: 8
    }
    Label {
        id: passwordLabel
        text: qsTr("Password:")
        anchors.bottom: passwordTextField.bottom
        anchors.left: parent.left
        padding: 8
    }
    TextField {
        id: passwordTextField
        anchors.top: usernameLabel.bottom
        anchors.left: passwordLabel.right
        padding: 8
        echoMode: TextInput.Password
    }
}
