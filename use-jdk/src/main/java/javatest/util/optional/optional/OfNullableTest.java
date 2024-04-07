package javatest.util.optional.optional;

import com.alibaba.fastjson2.JSON;
import com.xuegao.util.JsonUtil;
import common.model.temp.TempListBO;
import org.springframework.util.ObjectUtils;

import java.util.*;
import java.util.stream.Collectors;

/**
 * Passing a non-null argument to 'Optional'
 */
public class OfNullableTest {
    public static void main(String[] args) {
        test1();

        // test2();
        // List<String> nullList = null;
        // test3(nullList);
        // List<String> notNullList = new ArrayList<>();
        // test3(notNullList);
        // List<String> strList = Lists.newArrayList("1", "2");
        // test3(strList);

        // List<TempListBO> nullList = null;
        // test3(nullList);
        // List<TempListBO> notNullList = new ArrayList<>();
        // test3(notNullList);
        // List<TempListBO> strList = TempListBO.getList();
        // test3(strList);

        // List<TempListBO> test4_1 = test4(nullList);
        // System.out.println("test4_1 = " + JsonUtil.toJsonString(test4_1));
        // List<TempListBO> test4_2 = test4(Lists.newArrayList());
        // System.out.println("test4_2 = " + JsonUtil.toJsonString(test4_2));
        // List<TempListBO> test4_3 = test4(TempListBO.getList());
        // System.out.println("test4_3 = " + JsonUtil.toJsonString(test4_3));

        // List<TempListBO> test4_2_1 = test4_2(nullList);
        // System.out.println("test4_2_1 = " + JsonUtil.toJsonString(test4_2_1));
        // List<TempListBO> test4_2_2 = test4_2(Lists.newArrayList());
        // System.out.println("test4_2_2 = " + JsonUtil.toJsonString(test4_2_2));
        // List<TempListBO> test4_2_3 = test4_2(TempListBO.getList());
        // System.out.println("test4_2_3 = " + JsonUtil.toJsonString(test4_2_3));
    }

    private static void test1() {
        List<String> nullList = null;
        String nullListStr = Optional.ofNullable(nullList).map(JSON::toJSONString).orElse(null);
        System.out.println(nullListStr);

        List<String> notNullList = new ArrayList<>();
        String notNullListStr = Optional.ofNullable(notNullList).map(JSON::toJSONString).orElse(null);
        System.out.println(notNullListStr);

        // 如果过滤完了,list还是为空,那么直接报错
        // Optional<String> first = notNullList.stream().findFirst();
        // byte[] bytes = first.get().getBytes();
        // System.out.println(new String(bytes));

        notNullList.add("sdad");
        Optional<String> first = notNullList.stream().findFirst();
        if (first.isPresent()) {
            byte[] bytes = first.get().getBytes();
            System.out.println(new String(bytes));
        }
    }

    private static void test2(List<String> inputList) {
        String listStr = Optional.ofNullable(inputList).map(JSON::toJSONString).orElse(null);
        System.out.println(listStr);
    }

    private static void test3(List<TempListBO> tempListBOList) {
        System.out.println("==============================================");
        System.out.println("入参 = " + JsonUtil.toJsonString(tempListBOList));
        String nullListStr = Optional.ofNullable(tempListBOList).map(JSON::toJSONString).orElse(null);
        System.out.println("nullListStr = " + nullListStr);

        String notNullListStr = Optional.ofNullable(tempListBOList)
                .flatMap(c -> c.stream().filter(Objects::nonNull).findFirst())
                .map(TempListBO::getUsername)
                .orElse(null);
        System.out.println("notNullListStr = " + notNullListStr);

        TempListBO tempListBO = Optional.ofNullable(tempListBOList)
                .flatMap(c -> c.stream().filter(Objects::nonNull).findFirst())
                .orElse(null);
        System.out.println("obj = " + JsonUtil.toJsonString(tempListBO));

        Map<String, List<TempListBO>> resultMap = new HashMap<>(10);
        Optional.ofNullable(tempListBOList)
                .map(tempList -> {
                    System.out.println(" groupingBy ");
                    Map<String, List<TempListBO>> map = tempList.stream().collect(Collectors.groupingBy(TempListBO::getUsername));
                    resultMap.putAll(map);
                    return null;
                });
        System.out.println("resultMap = " + JsonUtil.toJsonString(resultMap));
    }

    public static TempListBO test4(List<TempListBO> tempListBOList) {
        return Optional.ofNullable(tempListBOList)
                .flatMap(c -> c.stream().filter(Objects::nonNull).findFirst())
                .orElse(null);
    }

    public static TempListBO test4_2(List<TempListBO> tempListBOList) {
        if (ObjectUtils.isEmpty(tempListBOList)) {
            return null;
        }
        return tempListBOList.get(0);
    }
}
