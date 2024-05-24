package javatest.io.file;


import java.security.SecureRandom;

public class FileTest {

    public static void main(String[] args) {
        String fileName1 = "a.vue";
        System.out.println(fileName1 + " = " + getSuffix(fileName1));
        // Assert.isTrue(getSuffix(fileName1).equals("VUE"), fileName1);

        String fileName2 = "a";
        System.out.println(fileName2 + " = " + getSuffix(fileName2));
        // Assert.isTrue(getSuffix(fileName2).equals(""), fileName2);

        String fileName3 = "a.png";
        System.out.println(fileName3 + " = " + getSuffix(fileName3));
        // Assert.isTrue(getSuffix(fileName3).equals("PNG"), fileName3);
    }

    public static String getSuffix(String fileName) {
        String extension = "";
        try {
            if (fileName.contains(".")) {
                extension = fileName.substring(fileName.lastIndexOf(".") + 1);
            }
        } catch (Exception e) {
            extension = "";
        }
        return extension.toUpperCase();
    }

}


