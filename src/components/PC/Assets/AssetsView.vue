<template>
  <div class="p-4">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-bold">资产概览</h1>
      <div class="flex space-x-2">
        <button @click="expandAll" class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 text-sm">
          展开全部
        </button>
        <button @click="collapseAll" class="px-3 py-1 bg-gray-500 text-white rounded hover:bg-gray-600 text-sm">
          收起全部
        </button>
      </div>
    </div>

    <!-- 路径导航 -->
    <div class="mb-4 text-sm text-gray-500">
      <span>资产 » </span>
      <span v-if="currentPath">{{ currentPath }}</span>
      <span v-else>全部账户</span>
    </div>

    <div class="bg-white rounded-lg shadow p-4">
      <div v-if="loading" class="flex justify-center items-center h-64">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
      </div>
      <div v-else-if="error" class="text-red-500 text-center">
        {{ error }}
      </div>
      <div v-else>
        <!-- 收入账户 -->
        <div class="mb-6" v-if="incomeAccounts.length > 0">
          <div @click="toggleSection('income')" class="flex items-center cursor-pointer mb-3">
            <h2 class="text-lg font-semibold text-green-600">收入账户</h2>
            <div class="ml-2 text-gray-500">
              <svg v-if="sectionExpanded.income" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
          <div v-if="sectionExpanded.income" class="ml-4">
            <account-tree-item
              v-for="account in incomeAccounts"
              :key="account.path || account.name"
              :account="account"
            />
          </div>
        </div>

        <!-- 支出账户 -->
        <div class="mb-6" v-if="expenseAccounts.length > 0">
          <div @click="toggleSection('expenses')" class="flex items-center cursor-pointer mb-3">
            <h2 class="text-lg font-semibold text-red-600">支出账户</h2>
            <div class="ml-2 text-gray-500">
              <svg v-if="sectionExpanded.expenses" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
          <div v-if="sectionExpanded.expenses" class="ml-4">
            <account-tree-item
              v-for="account in expenseAccounts"
              :key="account.path || account.name"
              :account="account"
            />
          </div>
        </div>

        <!-- 储蓄账户 -->
        <div class="mb-6" v-if="assetAccounts.length > 0">
          <div @click="toggleSection('assets')" class="flex items-center cursor-pointer mb-3">
            <h2 class="text-lg font-semibold text-blue-600">储蓄账户</h2>
            <div class="ml-2 text-gray-500">
              <svg v-if="sectionExpanded.assets" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
          <div v-if="sectionExpanded.assets" class="ml-4">
            <account-tree-item
              v-for="account in assetAccounts"
              :key="account.path || account.name"
              :account="account"
            />
          </div>
        </div>

        <!-- 负债账户 -->
        <div class="mb-6" v-if="liabilityAccounts.length > 0">
          <div @click="toggleSection('liabilities')" class="flex items-center cursor-pointer mb-3">
            <h2 class="text-lg font-semibold text-orange-600">负债账户</h2>
            <div class="ml-2 text-gray-500">
              <svg v-if="sectionExpanded.liabilities" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
          <div v-if="sectionExpanded.liabilities" class="ml-4">
            <account-tree-item
              v-for="account in liabilityAccounts"
              :key="account.path || account.name"
              :account="account"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, provide } from 'vue'
import AccountTreeItem from './AccountTreeItem.vue'

interface Account {
  name: string
  balance: number
  currency: string
  icon: string
  children?: Account[]
  displayName?: string
  path?: string
  level?: number  // 添加层级属性以跟踪嵌套深度
  expanded?: boolean // 节点的展开/折叠状态
  indent?: number // 缩进级别
  originalBalance?: number // 节点自身的原始余额
  totalBalance?: number // 包含所有子节点的总余额
  isParent?: boolean // 是否为父节点
}

const accounts = ref<Account[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const currentPath = ref('')

// 创建展开/折叠信号
const expandAllSignal = ref(0)
const collapseAllSignal = ref(0)

// 向子组件提供展开/折叠信号
provide('expandAllSignal', expandAllSignal)
provide('collapseAllSignal', collapseAllSignal)

const sectionExpanded = ref({
  income: true,
  expenses: true,
  assets: true,
  liabilities: true
})

// 格式化账户名称，将 :: 替换为更友好的分隔符同时保留层级信息
const formatAccountName = (account: Account, level = 0): Account => {
  const formattedAccount = { ...account };

  // 保存原始名称作为path
  formattedAccount.path = formattedAccount.name;

  // 分割路径
  const parts = formattedAccount.name.split('::');

  // 设置显示名称为最后一部分
  formattedAccount.displayName = parts[parts.length - 1];

  // 添加层级信息
  formattedAccount.level = level;

  // 默认展开状态：根和第一层展开，更深层级折叠
  formattedAccount.expanded = level < 2;

  // 格式化完整名称，用于显示
  formattedAccount.name = formattedAccount.name.replace(/::+/g, ' » ');

  if (formattedAccount.children && formattedAccount.children.length > 0) {
    formattedAccount.children = formattedAccount.children.map(child => formatAccountName(child, level + 1));
  }

  return formattedAccount;
};

const processAccounts = (rawAccounts: Account[]): Account[] => {
  const accountMap = new Map<string, Account>()
  const rootAccounts: Account[] = []

  // 首先创建所有账户的映射
  rawAccounts.forEach(account => {
    // 提取当前层级名称
    const parts = account.name.split('::');
    const displayName = parts[parts.length - 1];
    const level = parts.length - 1; // 计算层级深度

    accountMap.set(account.name, {
      ...account,
      displayName,
      path: account.name,
      level, // 添加层级信息
      expanded: level < 2, // 默认只展开前两层
      children: [],
      originalBalance: account.balance, // 保存原始余额
      isParent: false // 默认不是父节点
    })
  })

  // 确保父级路径都存在（处理跳跃层级的情况）
  accountMap.forEach((account, name) => {
    const parts = name.split('::');

    // 为每个中间路径创建虚拟节点（如果不存在）
    if (parts.length > 2) {
      // 从顶层开始，逐级检查并创建缺失的父节点
      for (let i = 1; i < parts.length - 1; i++) {
        const parentPath = parts.slice(0, i + 1).join('::');
        if (!accountMap.has(parentPath)) {
          // 创建虚拟父节点
          const parentDisplayName = parts[i];
          accountMap.set(parentPath, {
            name: parentPath,
            displayName: parentDisplayName,
            path: parentPath,
            balance: 0, // 虚拟节点默认余额为0
            currency: '', // 可以根据子节点设置
            icon: '',
            level: i,
            expanded: i < 2, // 默认只展开前两层
            children: [],
            originalBalance: 0,
            isParent: true // 这是一个父节点
          });
        }
      }
    }
  });

  // 构建树状结构
  accountMap.forEach((account, name) => {
    const parts = name.split('::')

    if (parts.length === 1) {
      // 这是根账户
      rootAccounts.push(account)
    } else {
      // 这是子账户，找到父账户并添加
      const parentName = parts.slice(0, parts.length - 1).join('::')
      const parent = accountMap.get(parentName)

      if (parent) {
        parent.children = parent.children || []
        parent.children.push(account)
        parent.isParent = true // 标记为父节点
      } else {
        // 如果找不到父账户，作为根账户处理
        rootAccounts.push(account)
      }
    }
  })

  // 计算每个父节点的余额总和
  const calculateTotalBalances = (accounts: Account[]): number => {
    let total = 0;

    accounts.forEach(account => {
      // 保存节点自身的原始余额
      account.originalBalance = account.balance;

      if (account.children && account.children.length > 0) {
        // 标记为父节点
        account.isParent = true;

        // 先递归计算所有子节点的总和
        const childrenTotal = calculateTotalBalances(account.children);

        // 设置子节点总余额
        account.totalBalance = childrenTotal;

        // 如果是虚拟节点（余额为0的父节点），使用子节点总和作为显示余额
        if (account.originalBalance === 0) {
          account.balance = childrenTotal;
        }

        // 确保所有子节点使用相同的货币单位
        if (!account.currency && account.children[0]?.currency) {
          account.currency = account.children[0].currency;
        }
      }

      // 累加当前节点的余额到总和
      total += account.originalBalance || 0;
    });

    return total;
  }

  calculateTotalBalances(rootAccounts)

  // 对每个节点的子节点进行排序（可选）
  const sortChildren = (accounts: Account[]) => {
    accounts.forEach(account => {
      if (account.children && account.children.length > 0) {
        account.children.sort((a, b) => a.name.localeCompare(b.name));
        sortChildren(account.children); // 递归排序子节点
      }
    });
  };

  // 计算每个节点的嵌套深度
  const calculateIndent = (accounts: Account[], currentIndent = 0) => {
    accounts.forEach(account => {
      account.indent = currentIndent;
      if (account.children && account.children.length > 0) {
        calculateIndent(account.children, currentIndent + 1);
      }
    });
  };

  sortChildren(rootAccounts);
  calculateIndent(rootAccounts);
  return rootAccounts;
}

const incomeAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('income'))
)

const expenseAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('expenses'))
)

const assetAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('Assets'))
)

const liabilityAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('Liabilities'))
)

// 展开所有节点
const expandAll = () => {
  // 展开所有分类
  Object.keys(sectionExpanded.value).forEach(key => {
    sectionExpanded.value[key as keyof typeof sectionExpanded.value] = true;
  });

  // 触发展开信号
  expandAllSignal.value += 1;
  console.log('展开全部被调用', expandAllSignal.value);
};

// 折叠所有节点
const collapseAll = () => {
  Object.keys(sectionExpanded.value).forEach(key => {
    sectionExpanded.value[key as keyof typeof sectionExpanded.value] = false;
  });

  // 触发折叠信号
  collapseAllSignal.value += 1;
};

// 切换分类的展开/折叠
const toggleSection = (section: keyof typeof sectionExpanded.value) => {
  sectionExpanded.value[section] = !sectionExpanded.value[section];
};

const getAssets = async () => {
  try {
    // TODO: 实现getAssets方法
    // 这里暂时使用模拟数据
    const rawAccounts = [
      {
        name: 'income::工资',
        balance: 10000,
        currency: 'CNY',
        icon: '/icons/salary.png'
      },
      {
        name: 'income::工资::1',
        balance: 10000,
        currency: 'CNY',
        icon: '/icons/salary.png'
      },
      {
        name: 'income::奖金',
        balance: 5000,
        currency: 'CNY',
        icon: '/icons/bonus.png'
      },
      {
        name: 'expenses::餐饮',
        balance: 2000,
        currency: 'CNY',
        icon: '/icons/food.png'
      },
      {
        name: 'expenses::交通',
        balance: 1000,
        currency: 'CNY',
        icon: '/icons/transport.png'
      },
      {
        name: 'expenses::购物::服装',
        balance: 1500,
        currency: 'CNY',
        icon: '/icons/clothes.png'
      },
      {
        name: 'expenses::购物::电子产品',
        balance: 3000,
        currency: 'CNY',
        icon: '/icons/electronics.png'
      },
      {
        name: 'expenses::住房::房租',
        balance: 2500,
        currency: 'CNY',
        icon: '/icons/rent.png'
      },
      {
        name: 'expenses::住房::水电',
        balance: 500,
        currency: 'CNY',
        icon: '/icons/utilities.png'
      },
      {
        name: 'Assets::储蓄卡',
        balance: 50000,
        currency: 'CNY',
        icon: '/icons/savings.png'
      },
      {
        name: 'Assets::投资::股票',
        balance: 30000,
        currency: 'CNY',
        icon: '/icons/stock.png'
      },
      {
        name: 'Assets::投资::基金',
        balance: 20000,
        currency: 'CNY',
        icon: '/icons/fund.png'
      },
      {
        name: 'Liabilities::信用卡',
        balance: 5000,
        currency: 'CNY',
        icon: '/icons/credit-card.png'
      },
      {
        name: 'Liabilities::房贷',
        balance: 500000,
        currency: 'CNY',
        icon: '/icons/mortgage.png'
      }
    ]
    accounts.value = processAccounts(rawAccounts)
  } catch (e) {
    error.value = '加载资产数据失败'
    console.error(e)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  getAssets()
})
</script>
